//! The `custom-print` crate helps you to define `print`, `println` and `dbg` macros
//! in wasm and customize them for other targets without any dependencies.
//!
//! # About
//!
//! This crate helps you to define `print`-like macros, `dbg` and `panic_hook`
//! on `wasm32-unknown-unknown` target without `wasm-bindgen` dependency.
//! Also, it can be used on another targets to override default std `write`-like macros,
//! add `try_` macros variants, or to specify panic hook function.
//! It works on stable Rust,
//! supports `no-alloc` and `no-std` environments and has no dependencies.
//!
//! In most cases it is suggested to use macros
//! [`define_macros`], [`define_macro`] or [`define_init_panic_hook`].
//! These macros define macros or functions with the specified names that use
//! [`FmtWriter`], [`FmtTryWriter`], [`ConcatWriter`], [`ConcatTryWriter`],
//! [`IoWriter`] or [`IoTryWriter`] with the specified closure, unsafe function or extern function.
//!
//! # Usage
//!
//! First, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! custom-print = "0.1.0"
//! ```
//!
//! This crate depends on the standard library by default.
//! To use this crate in a `#![no_std]` context but with heap-allocations enabled,
//! use `default-features = false` in your `Cargo.toml` as shown below:
//!
//! ```toml
//! [dependencies.custom-print]
//! version = "0.1.0"
//! default-features = false
//! features = ["alloc"]
//! ```
//!
//! # Examples
//!
//! An example with an extern functions that takes a UTF-8 chars pointer and byte length
//! with no `std` prelude:
#![cfg_attr(feature = "alloc", doc = " ```rust")]
#![cfg_attr(not(feature = "alloc"), doc = " ```rust,compile_fail")]
//! #![no_std]
//! extern crate std;
//!
//! # pub mod ffi {
//! #     #[no_mangle] pub extern "C" fn console_log(_: *const u8, _: usize) {}
//! #     #[no_mangle] pub extern "C" fn console_warn(_: *const u8, _: usize) {}
//! #     #[no_mangle] pub extern "C" fn console_error(_: *const u8, _: usize) {}
//! # }
//! #
//! custom_print::define_macros!({ print, println },
//!     concat, extern "C" fn console_log(_: *const u8, _: usize));
//! custom_print::define_macros!({ eprint, eprintln, dbg },
//!     concat, extern "C" fn console_warn(_: *const u8, _: usize));
//! custom_print::define_init_panic_hook!(
//!     concat, extern "C" fn console_error(_: *const u8, _: usize));
//!
//! fn main() {
//!     init_panic_hook();
//!     println!("println");
//!     print!("print");
//!     eprintln!("eprintln");
//!     eprint!("eprint");
//!     dbg!("dbg");
//! }
//! ```
//!
//! An example with a closure that takes an [`str`] reference
//! in `no_std` and `no_alloc` context:
#![cfg_attr(feature = "std", doc = " ```rust")]
#![cfg_attr(not(feature = "std"), doc = " ```rust,compile_fail")]
//! #![no_std]
//! custom_print::define_macros!({ print, println }, fmt, |_value: &str| { /* ... */ });
//!
//! # fn main() { // avoid clippy::needless_doctest_main false positive
//! fn main() {
//!     println!("println");
//!     print!("print");
//! }
//! # main();
//! # }
//! ```
//!
//! An example with a function that takes a [`c_char`] pointer and overriding
//! [`std::print`] and [`std::println`] functions:
#![cfg_attr(feature = "std", doc = " ```rust")]
#![cfg_attr(not(feature = "std"), doc = " ```rust,compile_fail")]
//! fn write(_value: *const std::os::raw::c_char) { /* ... */ }
//!
//! custom_print::define_macros!({ cprint, cprintln }, concat, crate::write);
//! macro_rules! print { ($($args:tt)*) => { cprint!($($args)*); } }
//! macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }
//!
//! fn main() {
//!     println!("println");
//!     print!("print");
//! }
//! ```
//!
//! # Macro expansion
//!
//! The example with [`define_macros`] and [`define_init_panic_hook`] with extern functions:
#![cfg_attr(feature = "alloc", doc = " ```rust")]
#![cfg_attr(not(feature = "alloc"), doc = " ```rust,compile_fail")]
//! #![no_std]
//! extern crate std;
//!
//! # pub mod ffi {
//! #     #[no_mangle] pub extern "C" fn console_log(_: *const u8, _: usize) {}
//! #     #[no_mangle] pub extern "C" fn console_warn(_: *const u8, _: usize) {}
//! #     #[no_mangle] pub extern "C" fn console_error(_: *const u8, _: usize) {}
//! # }
//! #
//! custom_print::define_macros!({ print, println, try_println },
//!     concat, extern "C" fn console_log(_: *const u8, _: usize));
//! custom_print::define_macros!({ eprint, eprintln, dbg },
//!     concat, extern "C" fn console_warn(_: *const u8, _: usize));
//! custom_print::define_init_panic_hook!(
//!     concat, extern "C" fn console_error(_: *const u8, _: usize));
//!
//! fn main() {
//!     init_panic_hook();
//!     println!("Greetings from println");
//!     let _ = try_println!("Greetings from try_println");
//!     eprintln!("Greetings from eprintln");
//!     let _ = dbg!("Greetings from dbg");
//! }
//! ```
//! partially expands to:
#![cfg_attr(feature = "alloc", doc = " ```rust")]
#![cfg_attr(not(feature = "alloc"), doc = " ```rust,compile_fail")]
//! # pub mod ffi {
//! #     #[no_mangle] pub extern "C" fn console_log(_: *const u8, _: usize) {}
//! #     #[no_mangle] pub extern "C" fn console_warn(_: *const u8, _: usize) {}
//! #     #[no_mangle] pub extern "C" fn console_error(_: *const u8, _: usize) {}
//! # }
//! #
//! fn init_panic_hook() {
//!     fn panic_hook(info: &::std::panic::PanicInfo<'_>) {
//!         ::core::writeln!(
//!             ::custom_print::ConcatWriter::from_closure({
//!                 extern "C" { fn console_error(_: *const u8, _: usize); }
//!                 |arg1: *const u8, arg2: usize| unsafe { console_error(arg1, arg2) }
//!             }),
//!             "{}",
//!             info
//!         ).expect("failed writing panic info");
//!     }
//!     ::std::panic::set_hook(::std::boxed::Box::new(panic_hook))
//! }
//!
//! fn main() {
//!     init_panic_hook();
//!
//!     ::core::writeln!(
//!         ::custom_print::ConcatWriter::from_closure({
//!             extern "C" { fn console_log(_: *const u8, _: usize); }
//!             |arg1: *const u8, arg2: usize| unsafe { console_log(arg1, arg2) }
//!         }),
//!         "Greetings from println"
//!     ).expect("failed writing");
//!
//!     let _ = ::core::writeln!(
//!         ::custom_print::ConcatTryWriter::from_closure({
//!             extern "C" { fn console_log(_: *const u8, _: usize); }
//!             |arg1: *const u8, arg2: usize| unsafe { console_log(arg1, arg2) }
//!         }),
//!         "Greetings from try_println"
//!     );
//!
//!     ::core::writeln!(
//!         ::custom_print::ConcatWriter::from_closure({
//!             extern "C" { fn console_warn(_: *const u8, _: usize); }
//!             |arg1: *const u8, arg2: usize| unsafe { console_warn(arg1, arg2) }
//!         }),
//!         "Greetings from eprintln"
//!     ).expect("failed writing");
//!
//!     let _ = ::custom_print::dbgwrite!(
//!         ::core::writeln,
//!         ::custom_print::ConcatWriter::from_closure({
//!             extern "C" { fn console_error(_: *const u8, _: usize); }
//!             |arg1: *const u8, arg2: usize| unsafe { console_error(arg1, arg2) }
//!         }),
//!         expect,
//!         ":?",
//!         "Greetings from dbg"
//!     );
//! }
//! ```
//!
//! # Feature Flags
//!
//! - `alloc` (implied by `std` so enabled by default):
//!   Enables [`WriteStringFn`] and [`ConcatWriter`] types.
//! - `std` (enabled by default):
//!   Enables [`IoWriter`], `{Try}Write{CStr|CString|CCharPtr}Fn`,
//!   [`define_panic_hook`] and [`define_init_panic_hook`].
//!
//! # Similar crates
//!
//! - [`web-log`]
//!   provides `print`, `println`, `eprint`, `eprintln`,
//!   requires wasm-bindgen.
//! - [`wasm-rs-dbg`]
//!   provides `dbg`,
//!   requires web-sys.
//! - [`console_log`]
//!   provides logging with `trace`, `debug`, `warn`, `error` etc.,
//!   requires log and web-sys.
//! - [`console_error_panic_hook`]
//!   provides panic_hook and panic hook set functions,
//!   requires wasm-bindgen.
//!
//! # Troubleshooting
//!
//! ## Macro name is ambiguous
//!
//! Errors like
//! `` `println` is ambiguous (macro-expanded name vs less macro-expanded name
//! from outer scope during import/macro resolution)``
//! occur because of the inability to overwrite standard rust macros
//! in [textual scope] with macro-expanded macros.
//!
//! Use can use proxy macros to replace `std` macros in [textual scope]:
//! ```rust
//! custom_print::define_macro!(cprintln, once: write_fn);
//! macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }
//! ```
//!
//! Alternatively, use can override macro in the [path-based scope]:
//! ```rust
//! custom_print::define_macro!(cprintln, once: write_fn);
//! use cprintln as println;
//! ```
//!
//! See [`define_macro`] for more details.
//!
//! ## Println, dbg and others do nothing in submodules
//!
//! It looks like you have overridden `print`-like macros in the [path-based scope],
//! but you have not overridden them in submodules.
//! Use proxy macros as it shown [above](#macro_name_is_ambiguous)
//! or do not forget to override it in submodules.
//! ```rust
//! custom_print::define_macro!(cprintln, once: write_fn);
//! use cprintln as println;
//! mod submodule {
//!     use cprintln as println;
//! }
//! ```
//!
//! You can always use [`cargo expand`] to find out where the problem is.
//!
//! ## The trait bound `[closure]: IntoWriteFn<_>` is not satisfied
//!
//! Errors like:
//! ``the trait bound `...: IntoWriteFn<_>` is not satisfied`` or
//! ``the trait bound `...: IntoTryWriteFn<_>` is not satisfied``,
//! with `note: required by ``...Writer::<F1>::from_closure` ``
//! errors occur because of the inability to determine
//! the appropriate type of wrapper for the closure.
//!
//! Specify closure arguments if you haven't already,
//! or use helper closure that takes acceptable arguments (`&str`, `&[u8]`, etc.)
//! and convert them to the arguments your function requires.
//!
//! # License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0
//!   ([LICENSE-APACHE](https://github.com/zheland/custom-print/blob/master/LICENSE-APACHE) or
//!   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
//! - MIT license
//!   ([LICENSE-MIT](https://github.com/zheland/custom-print/blob/master/LICENSE-MIT) or
//!   [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))
//!
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license,
//! shall be dual licensed as above, without any
//! additional terms or conditions.
//!
//! [`str`]: https://doc.rust-lang.org/std/str/index.html
//! [`c_char`]: https://doc.rust-lang.org/std/os/raw/type.c_char.html
//! [`std::print`]: https://doc.rust-lang.org/std/macro.print.html
//! [`std::println`]: https://doc.rust-lang.org/std/macro.println.html
//! [`define_macro`]: macro.define_macro.html
//! [`define_macros`]: macro.define_macros.html
//! [`define_panic_hook`]: macro.define_panic_hook.html
//! [`define_init_panic_hook`]: macro.define_init_panic_hook.html
//! [`WriteStringFn`]: struct.WriteStringFn.html
//! [`FmtWriter`]: struct.FmtWriter.html
//! [`FmtTryWriter`]: struct.FmtTryWriter.html
//! [`ConcatWriter`]: struct.ConcatWriter.html
//! [`ConcatTryWriter`]: struct.ConcatTryWriter.html
//! [`IoWriter`]: struct.IoWriter.html
//! [`IoTryWriter`]: struct.IoTryWriter.html
//! [`web-log`]: https://crates.io/crates/web-log
//! [`wasm-rs-dbg`]: https://crates.io/crates/wasm-rs-dbg
//! [`console_log`]: https://crates.io/crates/console_log
//! [`console_error_panic_hook`]: https://crates.io/crates/console_error_panic_hook
//! [`cargo expand`]: https://crates.io/crates/cargo-expand
//! [textual scope]: https://doc.rust-lang.org/reference/macros-by-example.html#scoping-exporting-and-importing
//! [path-based scope]: https://doc.rust-lang.org/reference/macros-by-example.html#scoping-exporting-and-importing

#![warn(
    clippy::all,
    rust_2018_idioms,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    missing_docs,
    trivial_casts,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod concat_try_writer;
#[cfg(feature = "alloc")]
mod concat_writer;
mod flush;
mod flush_fn;
mod fmt_try_writer;
mod fmt_writer;
mod into_try_write_fn;
mod into_write_fn;
#[cfg(feature = "std")]
mod io_try_writer;
#[cfg(feature = "std")]
mod io_writer;
mod macros;
mod never_error;
mod write_bytes;
mod write_fns;
mod write_str;

#[cfg(feature = "alloc")]
pub use concat_try_writer::{ConcatTryWriter, IntoConcatWriteResult};
#[cfg(feature = "alloc")]
pub use concat_writer::{ConcatWriter, ExpectConcatWriteResult};
pub use flush::Flush;
pub use flush_fn::FlushFn;
pub use fmt_try_writer::{FmtTryWriter, IntoFmtWriteResult};
pub use fmt_writer::{ExpectFmtWriteResult, FmtWriter};
pub use into_try_write_fn::IntoTryWriteFn;
pub use into_write_fn::IntoWriteFn;
#[cfg(feature = "std")]
pub use io_try_writer::{IntoIoFlushResult, IntoIoWriteResult, IoTryWriter};
#[cfg(feature = "std")]
pub use io_writer::{ExpectIoFlushResult, ExpectIoWriteResult, IoWriter};
pub use never_error::NeverError;
pub use write_bytes::WriteBytes;
#[cfg(feature = "alloc")]
pub use write_fns::WriteStringFn;
#[cfg(feature = "std")]
pub use write_fns::{
    TryWriteCCharPtrFn, TryWriteCStrFn, TryWriteCStringFn, WriteCCharPtrFn, WriteCStrFn,
    WriteCStringFn,
};
pub use write_fns::{WriteBytesFn, WriteLenPtrFn, WritePtrLenFn, WriteStrFn};
pub use write_str::{WriteStr, WriteStrAsBytes};
