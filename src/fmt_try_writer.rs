use core::fmt::{self, Arguments, Debug};

use crate::{IntoTryWriteFn, WriteBytes, WriteStr};

/// A writer that calls `write_str` for each formatted chunk, but do not require allocations.
///
/// Write function can return either `()` or [`fmt::Result`].
///
/// Writer propagates error to the caller if the write function returns `Result::Err`.
/// Note that the error context will be lost, because [`fmt::Error`]
/// does not support transmission of an error other than that an error occurred.
///
/// [`fmt::Error`]: https://doc.rust-lang.org/std/fmt/struct.Error.html
/// [`fmt::Result`]: https://doc.rust-lang.org/std/fmt/type.Result.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FmtTryWriter<F1>(F1);

/// A helper trait used by [`FmtTryWriter`]
/// to convert wrapped function result to [`fmt::Result`] with error propagation.
///
/// [`fmt::Result`]: https://doc.rust-lang.org/std/fmt/type.Result.html
pub trait IntoFmtWriteResult {
    /// Performs the conversion with error propagation.
    fn into_fmt_write_result(self) -> fmt::Result;
}

impl<F1> FmtTryWriter<F1>
where
    F1: WriteStr,
{
    /// Creates a new `FmtTryWriter` from an object that implements [`WriteStr`].
    pub fn new(write: F1) -> Self {
        Self(write)
    }

    /// Creates a new `FmtTryWriter` with a [`WriteStr`] wrapper
    /// deduced with [`IntoTryWriteFn`] by the closure signature and constructed from it.
    pub fn from_closure<F, Ts>(closure: F) -> Self
    where
        F: IntoTryWriteFn<Ts, TryWriteFn = F1>,
    {
        Self(closure.into_try_write_fn())
    }
}

impl<F1> FmtTryWriter<F1>
where
    Self: fmt::Write,
{
    /// Writes a formatted string into this writer, returning any error encountered.
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

impl<F1> fmt::Write for FmtTryWriter<F1>
where
    Self: WriteStr<Output = fmt::Result>,
{
    fn write_str(&mut self, buf: &str) -> fmt::Result {
        WriteStr::write_str(self, buf)
    }
}

impl<F1> WriteStr for FmtTryWriter<F1>
where
    F1: WriteStr,
    F1::Output: IntoFmtWriteResult,
{
    type Output = fmt::Result;

    fn write_str(&mut self, buf: &str) -> Self::Output {
        self.0.write_str(buf).into_fmt_write_result()
    }
}

impl<F1> WriteBytes for FmtTryWriter<F1>
where
    F1: WriteBytes,
    F1::Output: IntoFmtWriteResult,
{
    type Output = fmt::Result;

    fn write_bytes(&mut self, buf: &[u8]) -> Self::Output {
        self.0.write_bytes(buf).into_fmt_write_result()
    }
}

impl IntoFmtWriteResult for () {
    fn into_fmt_write_result(self) -> fmt::Result {
        Ok(())
    }
}

impl<E: Debug> IntoFmtWriteResult for Result<(), E> {
    fn into_fmt_write_result(self) -> fmt::Result {
        self.map_err(|_| fmt::Error)
    }
}
