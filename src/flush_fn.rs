use crate::Flush;

/// A wrapper for flush function `for<R> FnMut(*const c_char) -> R`.
///
/// It implements [`Flush`] trait and can be used in conjunction with IntoTryWriteFn trait to simplify type inference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FlushFn<F, R>(F)
where
    F: FnMut() -> R;

impl<F, R> FlushFn<F, R>
where
    F: FnMut() -> R,
{
    /// Creates a new `FlushFn` containing the given closure or function.
    pub fn new(closure: F) -> Self {
        Self(closure)
    }
}

impl<F, R> Flush for FlushFn<F, R>
where
    F: FnMut() -> R,
{
    type Output = R;
    fn flush(&mut self) -> Self::Output {
        self.0()
    }
}
