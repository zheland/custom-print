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

fn write(value: &str) {
    use std::string::ToString;
    let mut chunks = CHUNKS.lock().unwrap();
    chunks.push(value.to_string());
}

custom_print::define_macros!({ try_print, try_println }, concat, crate::write);

pub mod submodule {
    #[test]
    fn test_string_writer() {
        use crate::take_chunks;

        let result = try_print!("first");
        assert_eq!(result, Ok(()));
        assert_eq!(take_chunks(), &["first"]);
        let result = try_print!("first {}\nthird\n", "second");
        assert_eq!(result, Ok(()));
        assert_eq!(take_chunks(), &["first second\nthird\n"]);

        let result = try_println!();
        assert_eq!(result, Ok(()));
        assert_eq!(take_chunks(), &["\n"]);
        let result = try_println!("first");
        assert_eq!(result, Ok(()));
        assert_eq!(take_chunks(), &["first\n"]);
        let result = try_println!("first {}\nthird\n", "second");
        assert_eq!(result, Ok(()));
        assert_eq!(take_chunks(), &["first second\nthird\n\n"]);
    }
}
