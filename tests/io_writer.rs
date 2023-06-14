#![cfg(feature = "std")]
#![no_std]

extern crate std;

use core::str::from_utf8;
use std::string::String;
use std::sync::Mutex;
use std::vec::Vec;

use once_cell::sync::Lazy;

static CHUNKS: Lazy<Mutex<Vec<String>>> = Lazy::new(Mutex::default);

fn take_chunks() -> Vec<String> {
    use core::mem::take;
    take(&mut CHUNKS.lock().unwrap())
}

fn write_fn(value: &[u8]) {
    use std::string::ToString;
    let mut chunks = CHUNKS.lock().unwrap();
    chunks.push(from_utf8(value).unwrap().to_string());
}

#[inline(never)]
fn black_box<D>(input: D) -> D {
    unsafe {
        let output = std::ptr::read_volatile(&input);
        std::mem::forget(input);
        output
    }
}

custom_print::define_macros!({ print, println, dbg }, io, crate::write_fn);

pub mod submodule {
    #[test]
    fn test_io_writer() {
        use crate::{black_box, take_chunks};
        use std::format;
        use std::string::ToString;

        let file = file!();

        print!("first");
        assert_eq!(take_chunks(), &["first"]);
        print!("first {}\nthird\n", black_box("second"));
        assert_eq!(take_chunks(), &["first ", "second", "\nthird\n"]);

        println!();
        assert_eq!(take_chunks(), &["\n"]);
        println!("first");
        assert_eq!(take_chunks(), &["first\n"]);
        println!("first {}\nthird\n", black_box("second"));
        assert_eq!(take_chunks(), &["first ", "second", "\nthird\n\n"]);

        let second_var = "second";
        let (output, line) = (dbg!("first", second_var), line!().to_string());
        assert_eq!(output, ("first", "second"));
        let chunks = take_chunks();
        if chunks[0] == "[" {
            // rust version <= 1.70
            assert_eq!(
                chunks,
                &[
                    "[",
                    file,
                    ":",
                    &line,
                    "] ",
                    "\"first\"",
                    " = ",
                    "\"",
                    "first",
                    "\"",
                    "\n",
                    "[",
                    file,
                    ":",
                    &line,
                    "] ",
                    "second_var",
                    " = ",
                    "\"",
                    "second",
                    "\"",
                    "\n",
                ]
            );
        } else {
            // rust version >= 1.71
            assert_eq!(
                chunks,
                &[
                    &format!("[{file}:{line}] \"first\" = "),
                    "\"",
                    "first",
                    "\"",
                    "\n",
                    &format!("[{file}:{line}] second_var = "),
                    "\"",
                    "second",
                    "\"",
                    "\n",
                ]
            );
        }

        let second_var = "second";
        let (output, line) = (dbg!(("first", second_var)), line!().to_string());
        assert_eq!(output, ("first", "second"));
        let chunks = take_chunks();
        if chunks[0] == "[" {
            // rust version <= 1.70
            assert_eq!(
                chunks,
                &[
                    "[",
                    file,
                    ":",
                    &line,
                    "] ",
                    "(\"first\", second_var)",
                    " = ",
                    "(\n",
                    "    ",
                    "\"",
                    "first",
                    "\"",
                    ",\n",
                    "    ",
                    "\"",
                    "second",
                    "\"",
                    ",\n",
                    ")",
                    "\n"
                ]
            );
        } else {
            // rust version >= 1.71
            assert_eq!(
                chunks,
                &[
                    &format!("[{file}:{line}] (\"first\", second_var) = "),
                    "(\n",
                    "    ",
                    "\"",
                    "first",
                    "\"",
                    ",\n",
                    "    ",
                    "\"",
                    "second",
                    "\"",
                    ",\n",
                    ")",
                    "\n"
                ]
            );
        }
    }
}
