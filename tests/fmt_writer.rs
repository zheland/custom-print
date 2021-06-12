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

fn write_fn(value: &str) {
    use std::string::ToString;
    let mut chunks = CHUNKS.lock().unwrap();
    chunks.push(value.to_string());
}

custom_print::define_macros!({ print, println, dbg }, fmt, crate::write_fn);

pub mod submodule {
    #[test]
    fn test_fmt_writer() {
        use crate::take_chunks;
        use std::string::ToString;

        let file = ::core::file!();

        print!("first");
        assert_eq!(take_chunks(), &["first"]);
        print!("first {}\nthird\n", "second");
        assert_eq!(take_chunks(), &["first ", "second", "\nthird\n"]);

        println!();
        assert_eq!(take_chunks(), &["\n"]);
        println!("first");
        assert_eq!(take_chunks(), &["first\n"]);
        println!("first {}\nthird\n", "second");
        assert_eq!(take_chunks(), &["first ", "second", "\nthird\n\n"]);

        let second_var = "second";
        let (output, line) = (dbg!("first", second_var), ::core::line!().to_string());
        assert_eq!(output, ("first", "second"));
        assert_eq!(
            take_chunks(),
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

        let second_var = "second";
        let (output, line) = (dbg!(("first", second_var)), ::core::line!().to_string());
        assert_eq!(output, ("first", "second"));
        assert_eq!(
            take_chunks(),
            &[
                "[",
                file,
                ":",
                &line,
                "] ",
                "(\"first\", second_var)",
                " = ",
                "",
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
