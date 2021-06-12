#![cfg(feature = "std")]
#![no_std]

extern crate std;

use std::os::raw::c_char;
use std::string::String;
use std::sync::Mutex;
use std::vec::Vec;

use once_cell::sync::Lazy;

static CHUNKS: Lazy<Mutex<Vec<String>>> = Lazy::new(Mutex::default);

fn take_chunks() -> Vec<String> {
    use core::mem::take;
    take(&mut CHUNKS.lock().unwrap())
}

fn write_fn(c_str: *const c_char) {
    use std::ffi::CStr;
    use std::string::ToString;
    let mut chunks = CHUNKS.lock().unwrap();
    let c_str = unsafe { CStr::from_ptr(c_str) };
    let string = c_str.to_str().unwrap().to_string();
    chunks.push(string);
}

custom_print::define_macros!({ println, try_println }, fmt, crate::write_fn);

pub mod submodule {
    #[test]
    fn test_cchar_writer() {
        use crate::take_chunks;

        println!();
        assert_eq!(take_chunks(), &["\n"]);
        println!("first");
        assert_eq!(take_chunks(), &["first\n"]);
        println!("first {}\nthird\n", "second");
        assert_eq!(take_chunks(), &["first ", "second", "\nthird\n\n"]);
    }
}
