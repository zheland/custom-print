use crate::WriteBytes;

/// A trait for objects which can write [`str`] returning a specific output.
///
/// This trait is used by [`FmtWriter`] and [`ConcatWriter`] writers.
///
/// [`str`]: https://doc.rust-lang.org/std/str/index.html
/// [`FmtWriter`]: struct.FmtWriter.html
/// [`ConcatWriter`]: struct.ConcatWriter.html
pub trait WriteStr {
    /// The resulting type after writing.
    type Output;

    /// Performs byte writing.
    fn write_str(&mut self, buf: &str) -> Self::Output;
}

/// A trait for objects which should implement [`WriteStr`]
/// using their [`WriteBytes`] implementation.
pub trait WriteStrAsBytes: WriteBytes {}

impl<T> WriteStr for T
where
    T: WriteStrAsBytes,
{
    type Output = <T as WriteBytes>::Output;

    fn write_str(&mut self, buf: &str) -> Self::Output {
        self.write_bytes(buf.as_bytes())
    }
}
