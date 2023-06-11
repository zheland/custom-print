#![cfg(feature = "alloc")]
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

#[inline(never)]
fn black_box<D>(input: D) -> D {
    unsafe {
        let output = std::ptr::read_volatile(&input);
        std::mem::forget(input);
        output
    }
}

custom_print::define_macros!({ print, println, dbg }, concat, crate::write);

pub mod submodule {
    #[test]
    fn test_string_writer() {
        use crate::{black_box, take_chunks};
        use std::format;

        let file = ::core::file!();

        let () = print!("first");
        assert_eq!(take_chunks(), &["first"]);
        let () = print!("first {}\nthird\n", black_box("second"));
        assert_eq!(take_chunks(), &["first second\nthird\n"]);

        let () = println!();
        assert_eq!(take_chunks(), &["\n"]);
        let () = println!("first");
        assert_eq!(take_chunks(), &["first\n"]);
        let () = println!("first {}\nthird\n", black_box("second"));
        assert_eq!(take_chunks(), &["first second\nthird\n\n"]);

        let second_var = "second";
        let (output, line) = (dbg!("first", second_var), ::core::line!());
        assert_eq!(output, ("first", "second"));
        assert_eq!(
            take_chunks(),
            &[
                format!("[{}:{}] \"first\" = \"first\"\n", file, line),
                format!("[{}:{}] second_var = \"second\"\n", file, line),
            ]
        );

        let second_var = "second";
        let (output, line) = (dbg!(("first", second_var)), ::core::line!());
        assert_eq!(output, ("first", "second"));
        assert_eq!(
            take_chunks(),
            &[format!(
                "[{}:{}] (\"first\", second_var) = (\n    \"first\",\n    \"second\",\n)\n",
                file, line
            ),]
        );
    }
}
