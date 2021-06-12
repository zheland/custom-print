#![cfg(feature = "std")]

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
    let mut chunks = CHUNKS.lock().unwrap();
    chunks.push(value.to_string());
}

custom_print::define_macros!({ cprint, cprintln }, concat, crate::write);
use cprint as print;
use cprintln as println;

pub mod submodule {
    #[test]
    fn test_shadowing_with_use_in_submodule() {
        use crate::take_chunks;

        crate::print!("first");
        assert_eq!(take_chunks(), &["first"]);
        crate::print!("first {}\nthird\n", "second");
        assert_eq!(take_chunks(), &["first second\nthird\n"]);

        crate::println!();
        assert_eq!(take_chunks(), &["\n"]);
        crate::println!("first");
        assert_eq!(take_chunks(), &["first\n"]);
        crate::println!("first {}\nthird\n", "second");
        assert_eq!(take_chunks(), &["first second\nthird\n\n"]);
    }
}
