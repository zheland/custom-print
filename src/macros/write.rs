/// Calls another write macro with the specified writer, arguments and error-handling policy.
///
/// It propagates errors if the `try` policy is used.
///
/// # Panics
///
/// The macro panics if writing fails and the `expect` policy is used.
///
/// # Examples
///
/// ```rust
/// use core::fmt::Write;
/// let mut string = String::new();
///
/// assert_eq!(custom_print::write!(writeln, &mut string, expect, "first"), ());
/// assert_eq!(string, "first\n");
/// assert_eq!(custom_print::write!(writeln, &mut string, try, "second"), Ok(()));
/// assert_eq!(string, "first\nsecond\n");
/// ```
#[macro_export]
macro_rules! write {
    ( $macro:path, $writer:expr, expect $(, $($args:tt)*)? ) => {
        { $macro!($writer $(, $($args)*)?) }.expect("failed writing")
    };
    ( $macro:path, $writer:expr, try $(, $($args:tt)*)? ) => {
        { $macro!($writer $(, $($args)*)?) }
    };
}
