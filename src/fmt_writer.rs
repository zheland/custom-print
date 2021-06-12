use core::fmt::{self, Arguments, Debug};

use crate::{IntoWriteFn, WriteBytes, WriteStr};

/// A writer that calls `write_str` for each formatted chunk, but do not require allocations.
///
/// Write function can return either `()` or `for<E> `[`Result`]`<(), E>`.
///
/// # Panics
///
/// Writer panics if the write function returns `Result::Err`.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FmtWriter<F1>(F1);

/// A helper trait used by [`FmtWriter`]
/// to convert wrapped function result to [`fmt::Result`] with error unwrapping.
///
/// [`fmt::Result`]: https://doc.rust-lang.org/std/fmt/type.Result.html
pub trait ExpectFmtWriteResult {
    /// Performs the conversion with error unwrapping.
    fn expect_fmt_write_result(self) -> fmt::Result;
}

impl<F1> FmtWriter<F1>
where
    F1: WriteStr,
{
    /// Creates a new `FmtWriter` from an object that implements [`WriteStr`].
    pub fn new(write: F1) -> Self {
        Self(write)
    }

    /// Creates a new `FmtWriter` with a [`WriteStr`] wrapper
    /// deduced with [`IntoWriteFn`] by the closure signature and constructed from it.
    pub fn from_closure<F, Ts>(closure: F) -> Self
    where
        F: IntoWriteFn<Ts, WriteFn = F1>,
    {
        Self(closure.into_write_fn())
    }
}

impl<F1> FmtWriter<F1>
where
    Self: fmt::Write,
{
    /// Writes a formatted string into this writer.
    ///
    /// This method is primarily used to interface with the [`format_args!`] macro,
    /// but it is rare that this should explicitly be called.
    /// The [`write!`] macro should be favored to invoke this method instead.
    ///
    /// [`write!`]: https://doc.rust-lang.org/std/macro.write.html
    /// [`format_args!`]: https://doc.rust-lang.org/std/macro.format_args.html
    pub fn write_fmt(&mut self, args: Arguments<'_>) -> fmt::Result {
        fmt::Write::write_fmt(self, args)
    }
}

impl<F1> fmt::Write for FmtWriter<F1>
where
    Self: WriteStr<Output = fmt::Result>,
{
    fn write_str(&mut self, buf: &str) -> fmt::Result {
        WriteStr::write_str(self, buf)
    }
}

impl<F1> WriteStr for FmtWriter<F1>
where
    F1: WriteStr,
    F1::Output: ExpectFmtWriteResult,
{
    type Output = fmt::Result;

    fn write_str(&mut self, buf: &str) -> Self::Output {
        self.0.write_str(buf).expect_fmt_write_result()
    }
}

impl<F1> WriteBytes for FmtWriter<F1>
where
    F1: WriteBytes,
    F1::Output: ExpectFmtWriteResult,
{
    type Output = fmt::Result;

    fn write_bytes(&mut self, buf: &[u8]) -> Self::Output {
        self.0.write_bytes(buf).expect_fmt_write_result()
    }
}

impl ExpectFmtWriteResult for () {
    fn expect_fmt_write_result(self) -> fmt::Result {
        Ok(())
    }
}

impl<E: Debug> ExpectFmtWriteResult for Result<(), E> {
    fn expect_fmt_write_result(self) -> fmt::Result {
        self.expect("failed writing");
        Ok(())
    }
}
