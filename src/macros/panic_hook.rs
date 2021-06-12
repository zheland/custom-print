/// Defines `panic_hook` function that can be used as panic hook that uses the specified writer.
///
/// The first argument specify function name in the format `fn FUNC_NAME(...)`
/// and can be omitted to use the default name `panic_hook`.
///
/// # Examples
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// use once_cell::sync::Lazy;
/// use std::sync::Mutex;
///
/// static MESSAGE: Lazy<Mutex<String>> = Lazy::new(Mutex::default);
///
/// fn main() {
///     use std::panic::{catch_unwind, take_hook};
///
///     fn write(value: &str) {
///         let mut chunks = MESSAGE.lock().unwrap();
///         *chunks += value;
///     }
///
///     custom_print::define_panic_hook!(concat, write);
///     std::panic::set_hook(Box::new(panic_hook));
///
///     let result = catch_unwind(|| assert_eq!("foo", "bar"));
///     let _ = take_hook();
///     assert!(result.is_err());
///     let message = MESSAGE.lock().unwrap();
///
///     assert!(message.contains("panicked"));
///     assert!(message.contains("assertion failed"));
///     assert!(message.contains("\"foo\""));
///     assert!(message.contains("\"bar\""));
/// }
/// ```
#[macro_export]
macro_rules! define_panic_hook {
    ( $(#[$extern_meta:meta])* $vis:vis fn $name:ident(...), $($args:tt)* ) => {
        $(#[$extern_meta])*
        $vis fn $name(info: &::std::panic::PanicInfo<'_>) {
            ::core::writeln!($crate::define_writer!($($args)*), "{}", info)
                .expect("failed writing panic info");
        }
    };
    ( $($args:tt)* ) => {
        $crate::define_panic_hook!(fn panic_hook(...), $($args)*);
    };
}

/// Defines `init_panic_hook` function that set panic hook with the specified writer.
///
/// The first argument specify function name in the format `fn FUNC_NAME()`
/// and can be omitted to use the default name `init_panic_hook`.
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// use once_cell::sync::Lazy;
/// use std::sync::Mutex;
///
/// static MESSAGE: Lazy<Mutex<String>> = Lazy::new(Mutex::default);
///
/// fn main() {
///     use std::panic::{catch_unwind, take_hook};
///
///     fn write(value: &str) {
///         let mut chunks = MESSAGE.lock().unwrap();
///         *chunks += value;
///     }
///
///     custom_print::define_init_panic_hook!(concat, write);
///     init_panic_hook();
///
///     let result = catch_unwind(|| assert_eq!("foo", "bar"));
///     let _ = take_hook();
///     assert!(result.is_err());
///     let message = MESSAGE.lock().unwrap();
///
///     assert!(message.contains("panicked"));
///     assert!(message.contains("assertion failed"));
///     assert!(message.contains("\"foo\""));
///     assert!(message.contains("\"bar\""));
/// }
/// ```
#[macro_export]
macro_rules! define_init_panic_hook {
    ( $(#[$extern_meta:meta])* $vis:vis fn $name:ident(), $($args:tt)* ) => {
        $(#[$extern_meta])*
        $vis fn $name() {
            ::std::panic::set_hook(::std::boxed::Box::new(
                |info: &::std::panic::PanicInfo<'_>| {
                    ::core::writeln!($crate::define_writer!($($args)*), "{}", info)
                        .expect("failed writing panic info");
                }
            ))
        }
    };
    ( $($args:tt)* ) => {
        $crate::define_init_panic_hook!(fn init_panic_hook(), $($args)*);
    };
}

/// Sets `panic_hook` that uses the specified writer.
///
/// # Examples
///
#[cfg_attr(feature = "alloc", doc = "```rust")]
#[cfg_attr(not(feature = "alloc"), doc = "```rust,compile_fail")]
/// use once_cell::sync::Lazy;
/// use std::sync::Mutex;
///
/// static MESSAGE: Lazy<Mutex<String>> = Lazy::new(Mutex::default);
///
/// fn main() {
///     use std::panic::{catch_unwind, take_hook};
///
///     fn write(value: &str) {
///         let mut chunks = MESSAGE.lock().unwrap();
///         *chunks += value;
///     }
///
///     custom_print::init_panic_hook!(concat, write);
///
///     let result = catch_unwind(|| assert_eq!("foo", "bar"));
///     let _ = take_hook();
///     assert!(result.is_err());
///     let message = MESSAGE.lock().unwrap();
///
///     assert!(message.contains("panicked"));
///     assert!(message.contains("assertion failed"));
///     assert!(message.contains("\"foo\""));
///     assert!(message.contains("\"bar\""));
/// }
/// ```
#[macro_export]
macro_rules! init_panic_hook {
    ( $($args:tt)* ) => {
        ::std::panic::set_hook(::std::boxed::Box::new(
            |info: &::std::panic::PanicInfo<'_>| {
                ::core::writeln!($crate::define_writer!($($args)*), "{}", info)
                    .expect("failed writing panic info");
            }
        ))
    };
}
