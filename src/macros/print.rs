/// Defines `print` macro that defines a print macro that uses the specified writer.
///
/// The first argument specifies the generated macro name.
/// The writer itself is specified by the rest arguments with the [`define_writer`] macros.
///
/// See also [`define_println`] and [`define_try_print`] macros.
///
/// # Examples
///
/// ```rust
/// let mut string = String::new();
/// custom_print::define_print!(cprint, fmt, |value: &str| string += value);
///
/// assert_eq!(cprint!("value"), ());
/// assert_eq!(string, "value");
/// ```
///
/// [`define_writer`]: macro.define_writer.html
/// [`define_println`]: macro.define_println.html
/// [`define_try_print`]: macro.define_try_print.html
#[macro_export]
macro_rules! define_print {
    ( $( #[$meta:meta] )* $name:ident, $( $args:tt )* ) => {
        $crate::define_printlike!(
            $( #[$meta] )*
            $name,
            ::core::write,
            expect,
            $($args)*
        );
    };
}

/// Defines `println` macro that defines a println macro that uses the specified writer.
///
/// The first argument specifies the generated macro name.
/// The writer itself is specified by the rest arguments with the [`define_writer`] macros.
///
/// See also [`define_print`] and [`define_try_println`] macros.
///
/// # Examples
///
/// ```rust
/// let mut string = String::new();
/// custom_print::define_println!(cprintln, fmt, |value: &str| string += value);
///
/// assert_eq!(cprintln!("value"), ());
/// assert_eq!(string, "value\n");
/// ```
///
/// [`define_writer`]: macro.define_writer.html
/// [`define_print`]: macro.define_print.html
/// [`define_try_println`]: macro.define_try_println.html
#[macro_export]
macro_rules! define_println {
    ( $( #[$meta:meta] )* $name:ident, $( $args:tt )* ) => {
        $crate::define_printlike!(
            $( #[$meta] )*
            $name,
            ::core::writeln,
            expect,
            $($args)*
        );
    };
}

/// Defines `try_print` macro that defines a fallible print macro that uses the specified writer.
///
/// The first argument specifies the generated macro name.
/// The writer itself is specified by the rest arguments with the [`define_writer`] macros.
///
/// See also [`define_print`] and [`define_try_println`] macros.
///
/// # Examples
///
/// ```rust
/// let mut string = String::new();
/// custom_print::define_try_print!(try_print, fmt, |value: &str| string += value);
///
/// assert_eq!(try_print!("value"), Ok(()));
/// assert_eq!(string, "value");
/// ```
///
/// [`define_writer`]: macro.define_writer.html
/// [`define_print`]: macro.define_print.html
/// [`define_try_println`]: macro.define_try_println.html
#[macro_export]
macro_rules! define_try_print {
    ( $( #[$meta:meta] )* $name:ident, $( $args:tt )* ) => {
        $crate::define_printlike!(
            $( #[$meta] )*
            $name,
            ::core::write, try,
            $($args)*
        );
    };
}

/// Defines `try_println` macro that defines
/// a fallible println macro that uses the specified writer.
///
/// The first argument specifies the generated macro name.
/// The writer itself is specified by the rest arguments with the [`define_writer`] macros.
///
/// See also [`define_println`] and [`define_try_print`] macros.
///
/// # Examples
///
/// ```rust
/// let mut string = String::new();
/// custom_print::define_try_println!(try_println, fmt, |value: &str| string += value);
///
/// assert_eq!(try_println!("value"), Ok(()));
/// assert_eq!(string, "value\n");
/// ```
///
/// [`define_writer`]: macro.define_writer.html
/// [`define_println`]: macro.define_println.html
/// [`define_try_print`]: macro.define_try_print.html
#[macro_export]
macro_rules! define_try_println {
    ( $( #[$meta:meta] )* $name:ident, $( $args:tt )* ) => {
        $crate::define_printlike!(
            $( #[$meta] )*
            $name,
            ::core::writeln,
            try,
            $($args)*
        );
    };
}
