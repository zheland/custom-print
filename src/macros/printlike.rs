/// Defines a `print`-like macro with a given name that uses
/// specified write macro, error-handling policy and writer.
///
/// The writer itself is specified by the rest arguments with the [`define_writer`] macros.
///
/// # Examples
///
/// ```rust
/// let mut string = String::new();
/// custom_print::define_printlike!(cprintln, writeln, expect, fmt, |value: &str| string += value);
/// custom_print::define_printlike!(try_println, writeln, try, fmt, |value: &str| string += value);
///
/// assert_eq!(cprintln!("first"), ());
/// assert_eq!(string, "first\n");
/// assert_eq!(try_println!("second"), Ok(()));
/// assert_eq!(string, "first\nsecond\n");
/// ```
///
/// [`define_writer`]: macro.define_writer.html
#[macro_export]
macro_rules! define_printlike {
    (
        $( #[$meta:meta] )*
        $name:ident,
        $macro:path,
        expect,
        $( $args:tt )*
    ) => {
        $crate::_define_printlike_impl!(
            ($),
            $( #[$meta] )*,
            $name,
            $macro,
            expect,
            $crate::define_writer!($($args)*)
        );
    };
    (
        $( #[$meta:meta] )*
        $name:ident,
        $macro:path,
        try,
        $( $args:tt )*
    ) => {
        $crate::_define_printlike_impl!(
            ($),
            $( #[$meta] )*,
            $name,
            $macro,
            try,
            $crate::define_try_writer!($($args)*)
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _define_printlike_impl {
    (
        ($d:tt),
        $( #[$meta:meta] )*,
        $name:ident,
        $macro:path,
        $handler:tt,
        $writer:expr
    ) => {
        $( #[$meta] )*
        #[allow(unused_macros)]
        macro_rules! $name {
            // Dummy comment below is used to avoid rustfmt formatting bug.
            // Issue: https://github.com/rust-lang/rustfmt/issues/4609
            /* ================================================================================== */
            () => {
                $crate::write!($macro, $writer, $handler)
            };
            ($d ($d args:tt)+) => {
                $crate::write!($macro, $writer, $handler, $d ($d args)+)
            };
        }
    };
}
