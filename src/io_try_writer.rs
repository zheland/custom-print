use core::fmt::{Arguments, Debug};
use std::ffi;
use std::io;

use crate::{Flush, IntoTryWriteFn, WriteBytes, WriteStr};

/// A writer that uses `write_bytes` and has both `write` and `flush` methods.
///
/// It calls `write_bytes` for each formatted chunk like the [`FmtTryWriter`],
/// but provides write and flush methods that allows you to use [`BufWriter`], [`LineWriter`] etc.
///
/// Write function can return either `()`, `usize`, [`io::Result`]`<()>`, [`io::Result`]`<usize>`,
/// [`Result`]`<(), `[`ffi::NulError`]`>` or [`Result`]`<usize, `[`ffi::NulError`]`>`.
///
/// The `usize` itself or in `Result` indicates how many bytes were written.
/// `write_fmt` method that is used by [`write!`] and [`writeln!`]
/// will continuously call write until there is no more data to be written
/// or a non-[`ErrorKind::Interrupted`] kind is returned.
///
/// Flush function can return either `()` or [`io::Result`]`<()>`.
///
/// Writer propagates error to the caller if the write function returns `Result::Err`.
///
/// [`FmtTryWriter`]: struct.FmtTryWriter.html
/// [`write!`]: https://doc.rust-lang.org/std/macro.write.html
/// [`writeln!`]: https://doc.rust-lang.org/std/macro.writeln.html
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`io::Result`]: https://doc.rust-lang.org/std/io/type.Result.html
/// [`ffi::NulError`]: https://doc.rust-lang.org/std/ffi/struct.NulError.html
/// [`BufWriter`]: https://doc.rust-lang.org/std/io/struct.BufWriter.html
/// [`LineWriter`]: https://doc.rust-lang.org/std/io/struct.LineWriter.html
/// [`ErrorKind::Interrupted`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.Interrupted
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IoTryWriter<F1, F2>(F1, F2);

/// A helper trait used by [`IoTryWriter`] write method
/// to convert wrapped function result to [`io::Result`] with error propagation.
///
/// [`io::Result`]: https://doc.rust-lang.org/std/io/type.Result.html
pub trait IntoIoWriteResult {
    /// Performs the conversion with error propagation.
    fn into_io_write_result(self, buf: &[u8]) -> io::Result<usize>;
}

/// A helper trait used by [`IoTryWriter`] flush method
/// to convert wrapped function result to [`io::Result`] with error propagation.
///
/// [`io::Result`]: https://doc.rust-lang.org/std/io/type.Result.html
pub trait IntoIoFlushResult {
    /// Performs the conversion with error propagation.
    fn into_io_flush_result(self) -> io::Result<()>;
}

impl<F1, F2> IoTryWriter<F1, F2>
where
    F1: WriteStr,
    F2: Flush,
{
    /// Creates a new `IoTryWriter` from an object that implements [`WriteBytes`]
    /// and object that implements [`Flush`].
    pub fn new(write: F1, flush: F2) -> Self {
        Self(write, flush)
    }
}

impl<F1> IoTryWriter<F1, ()>
where
    F1: WriteStr,
{
    /// Creates a new `IoTryWriter` with a [`WriteBytes`] wrapper
    /// deduced with [`IntoTryWriteFn`] by the closure signature and constructed from it.
    pub fn from_closure<F, Ts>(closure: F) -> Self
    where
        F: IntoTryWriteFn<Ts, TryWriteFn = F1>,
    {
        Self(closure.into_try_write_fn(), ())
    }
}

impl<F1> IoTryWriter<F1, ()>
where
    Self: io::Write,
{
    /// Writes a formatted string into this writer, returning any error encountered.
    ///
    /// This method is primarily used to interface with the [`format_args!`] macro,
    /// but it is rare that this should explicitly be called.
    /// The [`write!`] macro should be favored to invoke this method instead.
    ///
    /// [`write!`]: https://doc.rust-lang.org/std/macro.write.html
    /// [`format_args!`]: https://doc.rust-lang.org/std/macro.format_args.html
    pub fn write_fmt(&mut self, args: Arguments<'_>) -> io::Result<()> {
        io::Write::write_fmt(self, args)
    }
}

impl<F1, F2> io::Write for IoTryWriter<F1, F2>
where
    Self: WriteBytes<Output = io::Result<usize>> + Flush<Output = io::Result<()>>,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        WriteBytes::write_bytes(self, buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Flush::flush(self)
    }
}

impl<F1, F2> WriteBytes for IoTryWriter<F1, F2>
where
    F1: WriteBytes,
    F1::Output: IntoIoWriteResult,
{
    type Output = io::Result<usize>;

    fn write_bytes(&mut self, buf: &[u8]) -> Self::Output {
        self.0.write_bytes(buf).into_io_write_result(buf)
    }
}

impl<F1, F2> Flush for IoTryWriter<F1, F2>
where
    F2: Flush,
    F2::Output: IntoIoFlushResult,
{
    type Output = io::Result<()>;

    fn flush(&mut self) -> Self::Output {
        self.1.flush().into_io_flush_result()
    }
}

impl IntoIoWriteResult for () {
    fn into_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }
}

impl IntoIoWriteResult for usize {
    fn into_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        let _ = buf;
        Ok(self)
    }
}

impl IntoIoWriteResult for Result<(), ffi::NulError> {
    fn into_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        self.map_or_else(
            |err| Err(io::Error::new(io::ErrorKind::InvalidData, err)),
            |_| Ok(buf.len()),
        )
    }
}

impl IntoIoWriteResult for Result<usize, ffi::NulError> {
    fn into_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        let _ = buf;
        self.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
}

impl IntoIoWriteResult for io::Result<()> {
    fn into_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        self.map(|_| buf.len())
    }
}

impl IntoIoWriteResult for io::Result<usize> {
    fn into_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        let _ = buf;
        self
    }
}

impl IntoIoFlushResult for () {
    fn into_io_flush_result(self) -> io::Result<()> {
        Ok(())
    }
}

impl IntoIoFlushResult for io::Result<()> {
    fn into_io_flush_result(self) -> io::Result<()> {
        self
    }
}
