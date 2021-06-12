/// A trait for objects which can write bytes returning a specific output.
///
/// This trait is used by [`IoWriter`].
///
/// [`IoWriter`]: struct.IoWriter.html
pub trait WriteBytes {
    /// The resulting type after writing.
    type Output;

    /// Performs byte writing.
    fn write_bytes(&mut self, buf: &[u8]) -> Self::Output;
}
