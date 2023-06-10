/// Defines a `dbg`-like macro with a given name that uses
/// specified write macro, error-handling policy, format string and writer.
///
/// The implementation of the generated `dbg`-like macro
/// is based on [`std::dbg`] macro implementation,
/// but the exact output printed by [`std::dbg`]
/// should not be relied upon and is subject to future changes.
///
/// If the `try` policy is used, it propagates write error and
/// returns values wrapper into `Result`.
///
/// The writer itself is specified by the rest arguments with the [`define_writer`] macros.
///
/// # Examples
///
/// ```rust
/// let mut string = String::new();
/// custom_print::define_dbglike!(cdbg, writeln, expect, ":?", fmt, |value: &str| string += value);
/// custom_print::define_dbglike!(try_dbg, writeln, try, ":?", fmt, |value: &str| string += value);
///
/// assert_eq!(cdbg!("first"), "first");
/// assert!(string.contains("\"first\""));
/// assert_eq!(try_dbg!("second"), Ok("second"));
/// assert!(string.contains("\"second\""));
/// ```
///
/// [`define_writer`]: macro.define_writer.html
#[macro_export]
macro_rules! define_dbglike {
    (
        $( #[$meta:meta] )*
        $name:ident,
        $macro:path,
        expect,
        $format:literal,
        $($args:tt)*
    ) => {
        $crate::_define_dbglike_impl!(
            ($),
            $( #[$meta] )*,
            $name,
            $macro,
            expect,
            $format,
            $crate::define_writer!($($args)*)
        );
    };
    (
        $( #[$meta:meta] )*
        $name:ident,
        $macro:path,
        try,
        $format:literal,
        $($args:tt)*
    ) => {
        $crate::_define_dbglike_impl!(
            ($),
            $( #[$meta] )*,
            $name,
            $macro,
            try,
            $format,
            $crate::define_try_writer!($($args)*)
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _define_dbglike_impl {
    (
        ($d:tt),
        $( #[$meta:meta] )*,
        $name:ident,
        $macro:path,
        $handler:tt,
        $format:literal,
        $writer:expr
    ) => {
        $( #[$meta] )*
        #[allow(unused_macros)]
        macro_rules! $name {
            // Dummy comment below is used to avoid rustfmt formatting bug.
            // Issue: https://github.com/rust-lang/rustfmt/issues/4609
            /* ================================================================================== */
            () => {
                $crate::dbgwrite!($macro, $writer, $handler, $format)
            };
            ($d ($d args:tt)+) => {
                $crate::dbgwrite!($macro, $writer, $handler, $format, $d ($d args)+)
            };
        }
    };
}
