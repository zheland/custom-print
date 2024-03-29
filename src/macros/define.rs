/// Defines multiple `print`-like and `dbg`-like macros.
///
/// The first argument braced in curly braces and contains
/// comma-seperated macro template names and optionally their custom macro names.
/// See the [`define_macro`] declaration for the list of available templates.
/// The rest tokens specified in `$($args)*` are used as input for [`define_writer`] macro.
///
/// Depending on the specified templates, the macro uses
/// [`define_print`], [`define_println`], [`define_dbg`], [`define_flush`],
/// [`define_try_print`], [`define_try_println`], [`define_try_dbg`] or [`define_try_flush`]
/// for each generated macro.
///
/// If you need to define a single `print`-like or `dbg`-like macro, use [`define_macro`].
///
/// # Macro ambiguity
///
/// When using std-prelude, std macros cannot be replaced in [textual scope] using this macro.
/// This is a consequence of the ambiguous precedence between
/// a macro-expanded macro and a macro from the outer scope.
/// Use alternative names like `cprint`, `ceprint`, `cprintln`, `ceprintln`, `cdbg`,
/// and then override std macros using proxy macro or use declaration.
///
/// Overriding with a proxy macro is a better way because
/// it overrides macros in [textual scope] and accordingly in all submodules:
/// ```
/// custom_print::define_macros!({ cprint, cprintln }, once: crate::write);
/// macro_rules! print { ($($args:tt)*) => { cprint!($($args)*); } }
/// macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }
/// # fn main() {}
/// mod submodule { /* println is already defined in all submodules */ }
/// ```
///
/// Alternatively, use can rebind macro from the [textual scope] to the [path-based scope],
/// but then it will be necessary not to forget to import macro into submodules scope:
/// ```
/// custom_print::define_macros!({ cprint, cprintln }, once: crate::write);
/// use cprint as print;
/// use cprintln as println;
/// # fn main() {}
/// mod submodule { use crate::{print, println}; /* ... */ }
/// ```
///
/// # Examples
///
/// An example with a simple string writer:
/// ```rust
/// use core::fmt::Write;
/// let mut string = String::new();
/// custom_print::define_macros!({ cprint, cprintln }, &mut string);
///
/// assert_eq!(cprintln!("first"), ());
/// assert_eq!(string, "first\n");
/// assert_eq!(cprint!("second"), ());
/// assert_eq!(string, "first\nsecond");
/// ```
///
/// An example with an extern functions that takes a UTF-8 chars pointer and byte length
/// and works in `no_std` context:
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// #![no_std]
/// extern crate std;
///
/// # pub mod ffi {
/// #     #[no_mangle] pub extern "C" fn console_log(_: *const u8, _: usize) {}
/// #     #[no_mangle] pub extern "C" fn console_warn(_: *const u8, _: usize) {}
/// # }
/// #
/// custom_print::define_macros!({ print, println },
///     concat, extern "C" fn console_log(_: *const u8, _: usize));
/// custom_print::define_macros!({ eprint, eprintln, dbg },
///     concat, extern "C" fn console_warn(_: *const u8, _: usize));
///
/// fn main() {
///     println!("println");
///     print!("print");
///     eprintln!("eprintln");
///     eprint!("eprint");
///     dbg!("dbg");
/// }
/// ```
///
/// An example with a closure that takes an [`str`] reference in `no_std` and `no_alloc`:
#[cfg_attr(feature = "std", doc = " ```rust")]
#[cfg_attr(not(feature = "std"), doc = " ```rust,compile_fail")]
/// #![no_std]
/// custom_print::define_macros!({ print, println }, fmt, |_value: &str| { /* ... */ });
///
/// fn main() {
///     println!("println");
///     print!("print");
/// }
/// ```
///
/// An example with a function that takes a [`c_char`] pointer and overriding
/// [`std::print`] and [`std::println`] functions:
#[cfg_attr(feature = "std", doc = "```rust")]
#[cfg_attr(not(feature = "std"), doc = "```rust,compile_fail")]
/// fn write(_value: *const std::os::raw::c_char) { /* ... */ }
///
/// custom_print::define_macros!({ cprint, cprintln }, concat, crate::write);
/// macro_rules! print { ($($args:tt)*) => { cprint!($($args)*); } }
/// macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }
///
/// fn main() {
///     println!("println");
///     print!("print");
/// }
/// ```
///
/// An example with `LineWriter` and flushing.
/// ```rust
/// use std::io::{self, LineWriter, Write};
/// use std::sync::Mutex;
///
/// let written: Mutex<Vec<u8>> = Mutex::default();
///
/// #[derive(Clone, Debug)]
/// struct CustomWriter<'a>(&'a Mutex<Vec<u8>>);
///
/// impl Write for CustomWriter<'_> {
///     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
///         let mut written = self.0.lock().unwrap();
///         written.extend_from_slice(buf);
///         Ok(buf.len())
///     }
///
///     fn flush(&mut self) -> io::Result<()> {
///         Ok(())
///     }
/// }
///
/// let custom_writer = CustomWriter(&written);
/// let mut line_writer = LineWriter::new(custom_writer);
///
/// custom_print::define_macros!({cprint, flush}, line_writer);
///
/// assert_eq!(cprint!("first,"), ());
/// assert_eq!(*written.lock().unwrap(), b"");
/// assert_eq!(cprint!("second\nthird,"), ());
/// assert_eq!(*written.lock().unwrap(), b"first,second\n");
/// assert_eq!(flush!(), ());
/// assert_eq!(*written.lock().unwrap(), b"first,second\nthird,");
/// ```
///
/// [`c_char`]: https://doc.rust-lang.org/std/os/raw/type.c_char.html
/// [textual scope]: https://doc.rust-lang.org/nightly/reference/macros-by-example.html#scoping-exporting-and-importing
/// [path-based scope]: https://doc.rust-lang.org/nightly/reference/macros-by-example.html#scoping-exporting-and-importing
/// [`define_macro`]: macro.define_macro.html
/// [`define_writer`]: macro.define_writer.html
/// [`define_print`]: macro.define_print.html
/// [`define_println`]: macro.define_println.html
/// [`define_dbg`]: macro.define_dbg.html
/// [`define_flush`]: macro.define_flush.html
/// [`define_try_print`]: macro.define_try_print.html
/// [`define_try_println`]: macro.define_try_println.html
/// [`define_try_dbg`]: macro.define_try_dbg.html
/// [`define_try_flush`]: macro.define_try_flush.html
#[macro_export]
macro_rules! define_macros {
    (
        $( #[$meta1:meta] )*
        { $( $( #[$meta2:meta] )* $template:ident $(as $name:ident)? ),* $(,)? },
        $( $args:tt )*
    ) => {
        $crate::_define_macros_impl!(
            $( #[$meta1] )*
            { $( $( #[$meta2] )* $template $( as $name )? ),* },
            $( $args )*
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _define_macros_impl {
    (
        $( #[$meta1:meta] )*
        { $( #[$meta2:meta] )* $template:ident $(as $name:ident)? $(, $($rest:tt)* )? },
        $( $args:tt )*
    ) => {
        $crate::define_macro!(
            $( #[$meta1] )*
            $( #[$meta2] )*
            $template $(as $name)?, $($args)*
        );
        $crate::_define_macros_impl!(
            $( #[$meta1] )*
            { $($($rest)*)? }, $($args)*
        );
    };
    ( $( #[$meta1:meta] )* { $(,)? } $(, $( $args:tt )* )? ) => {};
}

/// Defines custom `print`-like and `dbg`-like macro.
///
/// The first argument contains the macro name by which its template is determined,
/// or both the template name and the macro name using the syntax `template as name`.
/// The rest tokens specified in `$($args)*` are used as input for [`define_writer`] macro.
///
/// Depending on the specified template, the macro uses
/// [`define_print`], [`define_println`], [`define_dbg`], [`define_flush`],
/// [`define_try_print`], [`define_try_println`], [`define_try_dbg`] or [`define_try_flush`].
///
/// If you need to define multiple `print`-like and `dbg`-like macros, use [`define_macros`].
///
/// # Naming
///
/// The macros with the `try_` prefix are producing fallible write expressions.
/// The macros with the `e` prefix are proposed to be used with `stderr`-like writers.
/// The macros with the `c` prefix are proposed to be used
/// instead of the standard macros
/// or to shadow the standard macros in the following lines of code.
///
/// # Macro ambiguity
///
/// When using std-prelude, std macros cannot be replaced in [textual scope] using this macro.
/// This is a consequence of the ambiguous precedence between
/// a macro-expanded macro and a macro from the outer scope.
/// Use alternative names like `cprint`, `ceprint`, `cprintln`, `ceprintln`, `cdbg`,
/// and then override std macros using proxy macro or use declaration.
///
/// Overriding with a proxy macro is a better way because
/// it overrides macros in [textual scope] and accordingly in all submodules:
/// ```
/// custom_print::define_macro!(cprintln, once: crate::write);
/// macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }
/// # fn main() {}
/// mod submodule { /* println is already defined in all submodules */ }
/// ```
///
/// Alternatively, use can override macro in the [path-based scope],
/// but then it will be necessary not to forget to import macro into submodules scope:
/// ```
/// custom_print::define_macro!(cprintln, once: crate::write);
/// use cprintln as println;
/// # fn main() {}
/// mod submodule { use crate::println; /* ... */ }
/// ```
///
/// # Examples
///
/// An example with a simple string writer:
/// ```rust
/// use core::fmt::Write;
/// let mut string = String::new();
/// custom_print::define_macro!(cprint, &mut string);
///
/// assert_eq!(cprint!("value"), ());
/// assert_eq!(string, "value");
/// ```
///
/// An example with an extern functions that takes a UTF-8 chars pointer and byte length
/// and works in `no_std` context:
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// #![no_std]
/// extern crate std;
///
/// # pub mod ffi {
/// #     #[no_mangle] pub extern "C" fn console_log(_: *const u8, _: usize) {}
/// #     #[no_mangle] pub extern "C" fn console_warn(_: *const u8, _: usize) {}
/// # }
/// #
/// custom_print::define_macro!(println,
///     concat, extern "C" fn console_log(_: *const u8, _: usize));
/// custom_print::define_macro!(eprintln,
///     concat, extern "C" fn console_warn(_: *const u8, _: usize));
///
/// fn main() {
///     println!("println");
///     eprintln!("eprintln");
/// }
/// ```
///
/// An example with a closure that takes an [`str`] reference in `no_std` and `no_alloc`:
#[cfg_attr(feature = "std", doc = " ```rust")]
#[cfg_attr(not(feature = "std"), doc = " ```rust,compile_fail")]
/// #![no_std]
/// custom_print::define_macro!(println, fmt, |_value: &str| { /* ... */ });
///
/// fn main() {
///     println!("println");
/// }
/// ```
///
/// An example with a function that takes a [`c_char`] pointer and overriding
/// [`std::println`] function:
#[cfg_attr(feature = "std", doc = "```rust")]
#[cfg_attr(not(feature = "std"), doc = "```rust,compile_fail")]
/// fn write(_value: *const std::os::raw::c_char) { /* ... */ }
///
/// custom_print::define_macro!(cprintln, concat, crate::write);
/// macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }
///
/// fn main() {
///     println!("println");
/// }
/// ```
///
/// An example with `LineWriter` and flushing.
/// ```rust
/// use std::io::{self, LineWriter, Write};
/// use std::sync::Mutex;
///
/// let written: Mutex<Vec<u8>> = Mutex::default();
///
/// #[derive(Clone, Debug)]
/// struct CustomWriter<'a>(&'a Mutex<Vec<u8>>);
///
/// impl Write for CustomWriter<'_> {
///     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
///         let mut written = self.0.lock().unwrap();
///         written.extend_from_slice(buf);
///         Ok(buf.len())
///     }
///
///     fn flush(&mut self) -> io::Result<()> {
///         Ok(())
///     }
/// }
///
/// let custom_writer = CustomWriter(&written);
/// let mut line_writer = LineWriter::new(custom_writer);
///
/// custom_print::define_macro!(cprint, line_writer);
/// custom_print::define_macro!(flush, line_writer);
///
/// assert_eq!(cprint!("first,"), ());
/// assert_eq!(*written.lock().unwrap(), b"");
/// assert_eq!(cprint!("second\nthird,"), ());
/// assert_eq!(*written.lock().unwrap(), b"first,second\n");
/// assert_eq!(flush!(), ());
/// assert_eq!(*written.lock().unwrap(), b"first,second\nthird,");
/// ```
///
/// [`c_char`]: https://doc.rust-lang.org/std/os/raw/type.c_char.html
/// [textual scope]: https://doc.rust-lang.org/nightly/reference/macros-by-example.html#scoping-exporting-and-importing
/// [path-based scope]: https://doc.rust-lang.org/nightly/reference/macros-by-example.html#scoping-exporting-and-importing
/// [`define_writer`]: macro.define_writer.html
/// [`define_print`]: macro.define_print.html
/// [`define_println`]: macro.define_println.html
/// [`define_dbg`]: macro.define_dbg.html
/// [`define_flush`]: macro.define_flush.html
/// [`define_try_print`]: macro.define_try_print.html
/// [`define_try_println`]: macro.define_try_println.html
/// [`define_try_dbg`]: macro.define_try_dbg.html
/// [`define_try_flush`]: macro.define_try_flush.html
#[macro_export]
macro_rules! define_macro {
    ( $( #[$meta:meta] )* print       as $name:ident, $( $args:tt )* ) => {
        $crate::define_print!      ( $( #[$meta] )* $name, $( $args )* )
    };
    ( $( #[$meta:meta] )* println     as $name:ident, $( $args:tt )* ) => {
        $crate::define_println!    ( $( #[$meta] )* $name, $( $args )* )
    };
    ( $( #[$meta:meta] )* dbg         as $name:ident, $( $args:tt )* ) => {
        $crate::define_dbg!        ( $( #[$meta] )* $name, $( $args )* )
    };
    ( $( #[$meta:meta] )* flush       as $name:ident, $( $args:tt )* ) => {
        $crate::define_flush!      ( $( #[$meta] )* $name, $( $args )* )
    };
    ( $( #[$meta:meta] )* try_print   as $name:ident, $( $args:tt )* ) => {
        $crate::define_try_print!  ( $( #[$meta] )* $name, $( $args )* )
    };
    ( $( #[$meta:meta] )* try_println as $name:ident, $( $args:tt )* ) => {
        $crate::define_try_println!( $( #[$meta] )* $name, $( $args )* )
    };
    ( $( #[$meta:meta] )* try_dbg     as $name:ident, $( $args:tt )* ) => {
        $crate::define_try_dbg!    ( $( #[$meta] )* $name, $( $args )* )
    };
    ( $( #[$meta:meta] )* try_flush   as $name:ident, $( $args:tt )* ) => {
        $crate::define_try_flush!  ( $( #[$meta] )* $name, $( $args )* )
    };

    ( $( #[$meta:meta] )* print,        $( $args:tt )* ) => {
        $crate::define_print!  ( $( #[$meta] )* print,        $( $args )* );
    };
    ( $( #[$meta:meta] )* eprint,       $( $args:tt )* ) => {
        $crate::define_print!  ( $( #[$meta] )* eprint,       $( $args )* );
    };
    ( $( #[$meta:meta] )* cprint,       $( $args:tt )* ) => {
        $crate::define_print!  ( $( #[$meta] )* cprint,       $( $args )* );
    };
    ( $( #[$meta:meta] )* ceprint,      $( $args:tt )* ) => {
        $crate::define_print!  ( $( #[$meta] )* ceprint,      $( $args )* );
    };
    ( $( #[$meta:meta] )* println,      $( $args:tt )* ) => {
        $crate::define_println!( $( #[$meta] )* println,      $( $args )* );
    };
    ( $( #[$meta:meta] )* eprintln,     $( $args:tt )* ) => {
         $crate::define_println!( $( #[$meta] )* eprintln,     $( $args )* );
    };
    ( $( #[$meta:meta] )* cprintln,     $( $args:tt )* ) => {
        $crate::define_println!( $( #[$meta] )* cprintln,     $( $args )* );
    };
    ( $( #[$meta:meta] )* ceprintln,    $( $args:tt )* ) => {
        $crate::define_println!( $( #[$meta] )* ceprintln,    $( $args )* );
    };
    ( $( #[$meta:meta] )* dbg,          $( $args:tt )* ) => {
        $crate::define_dbg!    ( $( #[$meta] )* dbg,          $( $args )* );
    };
    ( $( #[$meta:meta] )* edbg,         $( $args:tt )* ) => {
        $crate::define_dbg!    ( $( #[$meta] )* edbg,         $( $args )* );
    };
    ( $( #[$meta:meta] )* cdbg,         $( $args:tt )* ) => {
        $crate::define_dbg!    ( $( #[$meta] )* cdbg,         $( $args )* );
    };
    ( $( #[$meta:meta] )* flush,        $( $args:tt )* ) => {
        $crate::define_flush!  ( $( #[$meta] )* flush,        $( $args )* );
    };
    ( $( #[$meta:meta] )* eflush,       $( $args:tt )* ) => {
        $crate::define_flush!  ( $( #[$meta] )* eflush,       $( $args )* );
    };

    ( $( #[$meta:meta] )* try_print,    $( $args:tt )* ) => {
        $crate::define_try_print!  ( $( #[$meta] )* try_print,    $( $args )* );
    };
    ( $( #[$meta:meta] )* try_eprint,   $( $args:tt )* ) => {
        $crate::define_try_print!  ( $( #[$meta] )* try_eprint,   $( $args )* );
    };
    ( $( #[$meta:meta] )* try_println,  $( $args:tt )* ) => {
        $crate::define_try_println!( $( #[$meta] )* try_println,  $( $args )* );
    };
    ( $( #[$meta:meta] )* try_eprintln, $( $args:tt )* ) => {
        $crate::define_try_println!( $( #[$meta] )* try_eprintln, $( $args )* );
    };
    ( $( #[$meta:meta] )* try_dbg,      $( $args:tt )* ) => {
        $crate::define_try_dbg!    ( $( #[$meta] )* try_dbg,      $( $args )* );
    };
    ( $( #[$meta:meta] )* try_edbg,     $( $args:tt )* ) => {
        $crate::define_try_dbg!    ( $( #[$meta] )* try_edbg,     $( $args )* );
    };
    ( $( #[$meta:meta] )* try_flush,    $( $args:tt )* ) => {
        $crate::define_try_flush!  ( $( #[$meta] )* try_flush,    $( $args )* );
    };
    ( $( #[$meta:meta] )* try_eflush,   $( $args:tt )* ) => {
        $crate::define_try_flush!  ( $( #[$meta] )* try_eflush,   $( $args )* );
    };
}
