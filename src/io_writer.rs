use core::fmt::{Arguments, Debug};
use std::io;

use crate::{Flush, IntoWriteFn, WriteBytes, WriteStr};

/// A writer that uses `write_bytes` and has both `write` and `flush` methods.
///
/// It calls `write_bytes` for each formatted chunk like the [`FmtWriter`],
/// but provides write and flush methods that allows you to use [`BufWriter`], [`LineWriter`] etc.
///
/// Write function can return either
/// `()`, `usize`, `for<E> `[`Result`]`<(), E>` or `for<E> `[`Result`]`<usize, E>`.
///
/// The `usize` itself or in `Result` indicates how many bytes were written.
/// `write_fmt` method that is used by [`write!`] and [`writeln!`]
/// will continuously call write until there is no more data to be written
/// or a non-[`ErrorKind::Interrupted`] kind is returned.
///
/// Flush function can return either `()` or [`for<E> Result<(), E>`].
///
/// # Panics
///
/// Writer panics if the write function returns `Result::Err`.
///
/// [`FmtWriter`]: struct.FmtWriter.html
/// [`write!`]: https://doc.rust-lang.org/std/macro.write.html
/// [`writeln!`]: https://doc.rust-lang.org/std/macro.writeln.html
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`BufWriter`]: https://doc.rust-lang.org/std/io/struct.BufWriter.html
/// [`LineWriter`]: https://doc.rust-lang.org/std/io/struct.LineWriter.html
/// [`ErrorKind::Interrupted`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.Interrupted
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IoWriter<F1, F2>(F1, F2);

/// A helper trait used by [`IoWriter`] write method
/// to convert wrapped function result to [`io::Result`] with error unwrapping.
///
/// [`io::Result`]: https://doc.rust-lang.org/std/io/type.Result.html
pub trait ExpectIoWriteResult {
    /// Performs the conversion with error unwrapping.
    fn expect_io_write_result(self, buf: &[u8]) -> io::Result<usize>;
}

/// A helper trait used by [`IoWriter`] flush method
/// to convert wrapped function result to [`io::Result`] with error unwrapping.
///
/// [`io::Result`]: https://doc.rust-lang.org/std/io/type.Result.html
pub trait ExpectIoFlushResult {
    /// Performs the conversion with error unwrapping.
    fn expect_io_flush_result(self) -> io::Result<()>;
}

impl<F1, F2> IoWriter<F1, F2>
where
    F1: WriteStr,
    F2: Flush,
{
    /// Creates a new `IoWriter` from an object that implements [`WriteBytes`]
    /// and object that implements [`Flush`].
    pub fn new(write: F1, flush: F2) -> Self {
        Self(write, flush)
    }
}

impl<F1> IoWriter<F1, ()>
where
    F1: WriteStr,
{
    /// Creates a new `IoWriter` with a [`WriteBytes`] wrapper
    /// deduced with [`IntoWriteFn`] by the closure signature and constructed from it.
    pub fn from_closure<F, Ts>(closure: F) -> Self
    where
        F: IntoWriteFn<Ts, WriteFn = F1>,
    {
        Self(closure.into_write_fn(), ())
    }
}

impl<F1> IoWriter<F1, ()>
where
    Self: io::Write,
{
    /// Writes a formatted string into this writer.
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

impl<F1, F2> io::Write for IoWriter<F1, F2>
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

impl<F1, F2> WriteBytes for IoWriter<F1, F2>
where
    F1: WriteBytes,
    F1::Output: ExpectIoWriteResult,
{
    type Output = io::Result<usize>;

    fn write_bytes(&mut self, buf: &[u8]) -> Self::Output {
        self.0.write_bytes(buf).expect_io_write_result(buf)
    }
}

impl<F1, F2> Flush for IoWriter<F1, F2>
where
    F2: Flush,
    F2::Output: ExpectIoFlushResult,
{
    type Output = io::Result<()>;

    fn flush(&mut self) -> Self::Output {
        self.1.flush().expect_io_flush_result()
    }
}

impl ExpectIoWriteResult for () {
    fn expect_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }
}

impl ExpectIoWriteResult for usize {
    fn expect_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        let _ = buf;
        Ok(self)
    }
}

impl<E: Debug> ExpectIoWriteResult for Result<(), E> {
    fn expect_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        self.expect("failed writing");
        Ok(buf.len())
    }
}

impl<E: Debug> ExpectIoWriteResult for Result<usize, E> {
    fn expect_io_write_result(self, buf: &[u8]) -> io::Result<usize> {
        let _ = buf;
        Ok(self.expect("failed writing"))
    }
}

impl ExpectIoFlushResult for () {
    fn expect_io_flush_result(self) -> io::Result<()> {
        Ok(())
    }
}

impl<E: Debug> ExpectIoFlushResult for Result<(), E> {
    fn expect_io_flush_result(self) -> io::Result<()> {
        self.expect("failed flushing");
        Ok(())
    }
}
