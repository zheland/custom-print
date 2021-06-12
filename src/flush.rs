/// A trait for objects which can flush written data on request.
///
/// This trait is used by [`IoWriter`].
///
/// [`IoWriter`]: struct.IoWriter.html
pub trait Flush {
    /// The resulting type after flushing.
    type Output;

    /// Performs flush.
    fn flush(&mut self) -> Self::Output;
}

impl Flush for () {
    type Output = ();

    #[inline]
    fn flush(&mut self) -> Self::Output {}
}
