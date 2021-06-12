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
macro_rules! print { ($($args:tt)*) => { cprint!($($args)*); } }
macro_rules! println { ($($args:tt)*) => { cprintln!($($args)*); } }

pub mod submodule {
    #[test]
    fn test_shadowing_with_macros_in_submodule() {
        use crate::take_chunks;

        print!("first");
        assert_eq!(take_chunks(), &["first"]);
        print!("first {}\nthird\n", "second");
        assert_eq!(take_chunks(), &["first second\nthird\n"]);

        println!();
        assert_eq!(take_chunks(), &["\n"]);
        println!("first");
        assert_eq!(take_chunks(), &["first\n"]);
        println!("first {}\nthird\n", "second");
        assert_eq!(take_chunks(), &["first second\nthird\n\n"]);
    }
}
