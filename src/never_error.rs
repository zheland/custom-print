use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// A helper error type that cannot be instantiated.
///
/// This error is used in [`ConcatWriter`] and [`ConcatTryWriter`]
/// there [`writeln!`] is supposed to return [`Result`]
/// but the `Result::Err` variant is not possible.
///
/// [`ConcatWriter`]: struct.ConcatWriter.html
/// [`ConcatTryWriter`]: struct.ConcatTryWriter.html
/// [`writeln!`]: https://doc.rust-lang.org/std/macro.writeln.html
/// [`Result`]: https://doc.rust-lang.org/std/result/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NeverError {}

impl Display for NeverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let _ = f;
        unreachable!();
    }
}

#[cfg(feature = "std")]
impl Error for NeverError {}
