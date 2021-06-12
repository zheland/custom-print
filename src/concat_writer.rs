use core::fmt::{Arguments, Debug};

use crate::{IntoWriteFn, NeverError, WriteBytes, WriteStr};

/// A writer that calls `write_str` once with a combined string.
///
/// Write function can return either `()` or `for<T, E> `[`Result`]`<T, E>`.
///
/// # Panics
///
/// Writer panics if the write function returns `Result::Err`.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConcatWriter<F1>(F1);

/// A helper trait used by [`ConcatWriter`]
/// to convert wrapped function result to [`Result`]`<T, NeverError>` with error unwrapping.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html

pub trait ExpectConcatWriteResult {
    /// The resulting type after convertion.
    type Output;

    /// Performs the conversion with error unwrapping.
    fn expect_concat_write_result(self) -> Self::Output;
}

impl<F1> ConcatWriter<F1>
where
    F1: WriteStr,
{
    /// Creates a new `ConcatWriter` from an object that implements [`WriteStr`].
    pub fn new(write: F1) -> Self {
        Self(write)
    }

    /// Creates a new `ConcatWriter` with a [`WriteStr`] wrapper
    /// deduced with [`IntoWriteFn`] by the closure signature and constructed from it.
    pub fn from_closure<F, Ts>(closure: F) -> Self
    where
        F: IntoWriteFn<Ts, WriteFn = F1>,
    {
        Self(closure.into_write_fn())
    }
}

impl<F1> ConcatWriter<F1>
where
    Self: WriteStr,
{
    /// Writes a formatted string into this writer.
    ///
    /// This method is primarily used to interface with the [`format_args!`] macro,
    /// but it is rare that this should explicitly be called.
    /// The [`write!`] macro should be favored to invoke this method instead.
    ///
    /// [`write!`]: https://doc.rust-lang.org/std/macro.write.html
    /// [`format_args!`]: https://doc.rust-lang.org/std/macro.format_args.html
    pub fn write_fmt(&mut self, args: Arguments<'_>) -> <Self as WriteStr>::Output {
        if let Some(buf) = args.as_str() {
            self.write_str(buf)
        } else {
            let buf = alloc::fmt::format(args);
            self.write_str(&buf)
        }
    }
}

impl<F1, Output> WriteStr for ConcatWriter<F1>
where
    F1: WriteStr,
    F1::Output: ExpectConcatWriteResult<Output = Output>,
{
    type Output = Output;

    fn write_str(&mut self, buf: &str) -> Output {
        self.0.write_str(buf).expect_concat_write_result()
    }
}

impl<F1, Output> WriteBytes for ConcatWriter<F1>
where
    F1: WriteBytes,
    F1::Output: ExpectConcatWriteResult<Output = Output>,
{
    type Output = Output;

    fn write_bytes(&mut self, buf: &[u8]) -> Output {
        self.0.write_bytes(buf).expect_concat_write_result()
    }
}

impl ExpectConcatWriteResult for () {
    type Output = Result<(), NeverError>;
    fn expect_concat_write_result(self) -> Self::Output {
        Ok(())
    }
}

impl<T, E: Debug> ExpectConcatWriteResult for Result<T, E> {
    type Output = Result<T, NeverError>;
    fn expect_concat_write_result(self) -> Self::Output {
        Ok(self.expect("failed writing"))
    }
}
