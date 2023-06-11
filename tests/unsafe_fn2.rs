#![cfg(feature = "std")]
#![no_std]

extern crate std;

use std::string::String;
use std::sync::Mutex;
use std::vec::Vec;

use once_cell::sync::Lazy;

static CHUNKS: Lazy<Mutex<Vec<String>>> = Lazy::new(Mutex::default);

fn take_chunks() -> Vec<String> {
    use core::mem::take;
    take(&mut CHUNKS.lock().unwrap())
}

unsafe fn write(ptr: *const u8, len: usize) {
    use core::slice;
    use core::str;
    use std::string::ToString;

    let mut chunks = CHUNKS.lock().unwrap();
    let value = slice::from_raw_parts(ptr, len);
    let value = str::from_utf8(value).unwrap();
    chunks.push(value.to_string());
}

#[inline(never)]
fn black_box<D>(input: D) -> D {
    unsafe {
        let output = std::ptr::read_volatile(&input);
        std::mem::forget(input);
        output
    }
}

custom_print::define_macros!({ print, println },
    concat, unsafe fn (crate::write)(*const u8, usize));

pub mod submodule {
    #[test]
    fn test_string_writer() {
        use crate::{black_box, take_chunks};

        print!("first");
        assert_eq!(take_chunks(), &["first"]);
        print!("first {}\nthird\n", black_box("second"));
        assert_eq!(take_chunks(), &["first second\nthird\n"]);

        println!();
        assert_eq!(take_chunks(), &["\n"]);
        println!("first");
        assert_eq!(take_chunks(), &["first\n"]);
        println!("first {}\nthird\n", black_box("second"));
        assert_eq!(take_chunks(), &["first second\nthird\n\n"]);
    }
}
