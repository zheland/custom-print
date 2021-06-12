/// Defines `dbg` macro that prints and returns the value
/// of a given expression for quick and dirty debugging.
///
/// The implementation of the generated `dbg`-like macro
/// is based on [`std::dbg`] macro implementation,
/// but the exact output printed by [`std::dbg`]
/// should not be relied upon and is subject to future changes.
///
/// The first argument specifies the generated macro name.
/// The writer itself is specified by the rest arguments with the [`define_writer`] macros.
///
/// Use [`define_try_dbg`] if you need to define a fallible dbg macros.
///
/// # Examples
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// let mut string = String::new();
/// custom_print::define_dbg!(cdbg, concat, |value: &str| string += value);
///
/// assert_eq!(cdbg!("value"), "value");
/// assert!(string.contains("\"value\""));
/// ```
#[macro_export]
macro_rules! define_dbg {
    ( $name:ident, $($args:tt)* ) => {
        $crate::define_dbglike!( $name, ::core::writeln, expect, ":#?", $($args)* );
    };
}

/// Defines `try_dbg` macro that prints and returns the value
/// of a given expression wrapped in for quick and dirty debugging
/// or return an error if write failed.
///
/// The implementation of the generated `dbg`-like macro
/// is based on [`std::dbg`] macro implementation,
/// but the exact output printed by [`std::dbg`]
/// should not be relied upon and is subject to future changes.
///
/// The first argument specifies the generated macro name.
/// The writer itself is specified by the rest arguments with the [`define_writer`] macros.
///
/// Use [`define_dbg`] if you need to define a non-fallible dbg macros.
///
/// # Examples
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// let mut string = String::new();
/// custom_print::define_try_dbg!(try_dbg, concat, |value: &str| string += value);
///
/// assert_eq!(try_dbg!("value"), Ok("value"));
/// assert!(string.contains("\"value\""));
/// ```
#[macro_export]
macro_rules! define_try_dbg {
    ( $name:ident, $($args:tt)* ) => {
        $crate::define_dbglike!( $name, ::core::writeln, try, ":#?", $($args)* );
    };
}
