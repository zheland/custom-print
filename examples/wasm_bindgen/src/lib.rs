#![warn(
    clippy::all,
    rust_2018_idioms,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    trivial_casts,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![no_std]
extern crate std;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(_: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(_: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(_: &str);
}

custom_print::define_macros!({ print, println }, concat, unsafe fn (crate::log)(&str));
custom_print::define_macros!({ eprint, eprintln, dbg }, concat, unsafe fn (crate::warn)(&str));
custom_print::define_init_panic_hook!(concat, unsafe fn error(&str));

pub mod submodule {
    use wasm_bindgen::prelude::*;
    #[cfg(test)]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen]
    pub fn start() {
        crate::init_panic_hook();
        println!("println");
        print!("print");
        eprintln!("eprintln");
        eprint!("eprint");
        dbg!("dbg");
        panic!("panic");
    }

    #[cfg(test)]
    #[wasm_bindgen_test]
    pub fn test_start() {
        crate::init_panic_hook();
        println!("succeed");
    }
}
