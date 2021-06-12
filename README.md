# custom-print

![CI](https://github.com/zheland/custom-print/workflows/CI/badge.svg)
[![Latest Version](https://img.shields.io/crates/v/custom-print.svg)](https://crates.io/crates/custom-print)
[![Documentation](https://docs.rs/custom-print/badge.svg)](https://docs.rs/custom-print)
[![GitHub license](https://img.shields.io/crates/l/custom-print)](https://github.com/zheland/custom-print/#license)
[![Rust Version](https://img.shields.io/badge/rustc-1.52+-lightgray.svg)](https://blog.rust-lang.org/2020/01/30/Rust-1.52.0.html)

The `custom-print` crate helps you to define `print`, `println` and `dbg` macros
in wasm and customize them for other targets without any dependencies.

## About

This crate helps you to define `print`-like macros, `dbg` and `panic_hook`
on `wasm32-unknown-unknown` target without `wasm-bindgen` dependency.
Also, it can be used on another targets to override default std `write`-like macros,
add `try_` macros variants, or to specify panic hook function.
It works on stable Rust,
supports `no-alloc` and `no-std` environments and has no dependencies.

In most cases it is suggested to use macros
[`define_macros`], [`define_macro`] or [`define_init_panic_hook`].
These macros define macros or functions with the specified names that use
[`FmtWriter`], [`FmtTryWriter`], [`ConcatWriter`], [`ConcatTryWriter`],
[`IoWriter`] or [`IoTryWriter`] with the specified closure, unsafe function or extern function.

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
custom-print = "0.1.0"
```

This crate depends on the standard library by default.
To use this crate in a `#![no_std]` context but with heap-allocations enabled,
use `default-features = false` in your `Cargo.toml` as shown below:

```toml
[dependencies.custom-print]
version = "0.1.0"
default-features = false
features = ["alloc"]
```

## Examples

An example with an extern functions that takes a UTF-8 chars pointer and byte length
with no `std` prelude:
```rust
#![no_std]
extern crate std;

custom_print::define_macros!({ print, println },
    concat, extern "C" fn console_log(_: *const u8, _: usize));
custom_print::define_macros!({ eprint, eprintln, dbg },
    concat, extern "C" fn console_warn(_: *const u8, _: usize));
custom_print::define_init_panic_hook!(
    concat, extern "C" fn console_error(_: *const u8, _: usize));

fn main() {
    init_panic_hook();
    println!("println");
    print!("print");
    eprintln!("eprintln");
    eprint!("eprint");
    dbg!("dbg");
}
```

An example with a closure that takes an [`str`] reference
in `no_std` and `no_alloc` context:
```rust
#![no_std]
custom_print::define_macros!({ print, println }, fmt, |_value: &str| { /* ... */ });

fn main() {
    println!("println");
    print!("print");
}
```

An example with a function that takes a [`c_char`] pointer and overriding
[`std::print`] and [`std::println`] functions:
```rust
fn write(_value: *const std::os::raw::c_char) { /* ... */ }

custom_print::define_macros!({ cprint, cprintln }, concat, crate::write);
macro_rules! print { ($($args:tt)*) => { cprint!($($args)*); } }
macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }

fn main() {
    println!("println");
    print!("print");
}
```

## Macro expansion

The example with [`define_macros`] and [`define_init_panic_hook`] with extern functions:
```rust
#![no_std]
extern crate std;

custom_print::define_macros!({ print, println, try_println },
    concat, extern "C" fn console_log(_: *const u8, _: usize));
custom_print::define_macros!({ eprint, eprintln, dbg },
    concat, extern "C" fn console_warn(_: *const u8, _: usize));
custom_print::define_init_panic_hook!(
    concat, extern "C" fn console_error(_: *const u8, _: usize));

fn main() {
    init_panic_hook();
    println!("Greetings from println");
    let _ = try_println!("Greetings from try_println");
    eprintln!("Greetings from eprintln");
    let _ = dbg!("Greetings from dbg");
}
```
partially expands to:
```rust
fn init_panic_hook() {
    fn panic_hook(info: &::std::panic::PanicInfo<'_>) {
        ::core::writeln!(
            ::custom_print::ConcatWriter::from_closure({
                extern "C" { fn console_error(_: *const u8, _: usize); }
                |arg1: *const u8, arg2: usize| unsafe { console_error(arg1, arg2) }
            }),
            "{}",
            info
        ).expect("failed writing panic info");
    }
    ::std::panic::set_hook(::std::boxed::Box::new(panic_hook))
}

fn main() {
    init_panic_hook();

    ::core::writeln!(
        ::custom_print::ConcatWriter::from_closure({
            extern "C" { fn console_log(_: *const u8, _: usize); }
            |arg1: *const u8, arg2: usize| unsafe { console_log(arg1, arg2) }
        }),
        "Greetings from println"
    ).expect("failed writing");

    let _ = ::core::writeln!(
        ::custom_print::ConcatTryWriter::from_closure({
            extern "C" { fn console_log(_: *const u8, _: usize); }
            |arg1: *const u8, arg2: usize| unsafe { console_log(arg1, arg2) }
        }),
        "Greetings from try_println"
    );

    ::core::writeln!(
        ::custom_print::ConcatWriter::from_closure({
            extern "C" { fn console_warn(_: *const u8, _: usize); }
            |arg1: *const u8, arg2: usize| unsafe { console_warn(arg1, arg2) }
        }),
        "Greetings from eprintln"
    ).expect("failed writing");

    let _ = ::custom_print::dbgwrite!(
        ::core::writeln,
        ::custom_print::ConcatWriter::from_closure({
            extern "C" { fn console_error(_: *const u8, _: usize); }
            |arg1: *const u8, arg2: usize| unsafe { console_error(arg1, arg2) }
        }),
        expect,
        ":?",
        "Greetings from dbg"
    );
}
```

## Documentation

[API Documentation]

## Feature Flags

- `alloc` (implied by `std` so enabled by default):
  Enables [`WriteStringFn`] and [`ConcatWriter`] types.
- `std` (enabled by default):
  Enables [`IoWriter`], `{Try}Write{CStr|CString|CCharPtr}Fn`,
  [`define_panic_hook`] and [`define_init_panic_hook`].

## Similar crates

- [`web-log`]
  provides `print`, `println`, `eprint`, `eprintln`,
  requires wasm-bindgen.
- [`wasm-rs-dbg`]
  provides `dbg`,
  requires web-sys.
- [`console_log`]
  provides logging with `trace`, `debug`, `warn`, `error` etc.,
  requires log and web-sys.
- [`console_error_panic_hook`]
  provides panic_hook and panic hook set functions,
  requires wasm-bindgen.

## Troubleshooting

### Macro name is ambiguous

Errors like
`` `println` is ambiguous (macro-expanded name vs less macro-expanded name
from outer scope during import/macro resolution)``
occur because of the inability to overwrite standard rust macros
in [textual scope] with macro-expanded macros.

Use can use proxy macros to replace `std` macros in [textual scope]:
```rust
custom_print::define_macro!(cprintln, once: write_fn);
macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }
```

Alternatively, use can override macro in the [path-based scope]:
```rust
custom_print::define_macro!(cprintln, once: write_fn);
use cprintln as println;
```

See [`define_macro`] for more details.

### Println, dbg and others do nothing in submodules

It looks like you have overridden `print`-like macros in the [path-based scope],
but you have not overridden them in submodules.
Use proxy macros as it shown [above](#macro_name_is_ambiguous)
or do not forget to override it in submodules.
```rust
custom_print::define_macro!(cprintln, once: write_fn);
use cprintln as println;
mod submodule {
    use cprintln as println;
}
```

You can always use [`cargo expand`] to find out where the problem is.

### The trait bound `[closure]: IntoWriteFn<_>` is not satisfied

Errors like:
``the trait bound `...: IntoWriteFn<_>` is not satisfied`` or
``the trait bound `...: IntoTryWriteFn<_>` is not satisfied``,
with `note: required by ``...Writer::<F1>::from_closure` ``
errors occur because of the inability to determine
the appropriate type of wrapper for the closure.

Specify closure arguments if you haven't already,
or use helper closure that takes acceptable arguments (`&str`, `&[u8]`, etc.)
and convert them to the arguments your function requires.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any
additional terms or conditions.

[API Documentation]: https://docs.rs/custom-print
[`str`]: https://doc.rust-lang.org/std/str/index.html
[`c_char`]: https://doc.rust-lang.org/std/os/raw/type.c_char.html
[`std::print`]: https://doc.rust-lang.org/std/macro.print.html
[`std::println`]: https://doc.rust-lang.org/std/macro.println.html
[`define_macro`]: https://docs.rs/custom-print/*/custom_print/macro.define_macro.html
[`define_macros`]: https://docs.rs/custom-print/*/custom_print/macro.define_macros.html
[`define_panic_hook`]: https://docs.rs/custom-print/*/custom_print/macro.define_panic_hook.html
[`define_init_panic_hook`]: https://docs.rs/custom-print/*/custom_print/macro.define_init_panic_hook.html
[`WriteStringFn`]: https://docs.rs/custom-print/*/custom_print/struct.WriteStringFn.html
[`FmtWriter`]: https://docs.rs/custom-print/*/custom_print/struct.FmtWriter.html
[`FmtTryWriter`]: https://docs.rs/custom-print/*/custom_print/struct.FmtTryWriter.html
[`ConcatWriter`]: https://docs.rs/custom-print/*/custom_print/struct.ConcatWriter.html
[`ConcatTryWriter`]: https://docs.rs/custom-print/*/custom_print/struct.ConcatTryWriter.html
[`IoWriter`]: https://docs.rs/custom-print/*/custom_print/struct.IoWriter.html
[`IoTryWriter`]: https://docs.rs/custom-print/*/custom_print/struct.IoTryWriter.html
[`web-log`]: https://crates.io/crates/web-log
[`wasm-rs-dbg`]: https://crates.io/crates/wasm-rs-dbg
[`console_log`]: https://crates.io/crates/console_log
[`console_error_panic_hook`]: https://crates.io/crates/console_error_panic_hook
[`cargo expand`]: https://crates.io/crates/cargo-expand
[textual scope]: https://doc.rust-lang.org/reference/macros-by-example.html#scoping-exporting-and-importing
[path-based scope]: https://doc.rust-lang.org/reference/macros-by-example.html#scoping-exporting-and-importing
