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

custom_print::define_macro!(try_dbg, concat, crate::write);

pub mod submodule {
    #[test]
    fn test_string_writer() {
        use crate::take_chunks;
        use std::format;

        let file = ::core::file!();
        let second_var = "second";

        let (output, line) = (try_dbg!(), ::core::line!());
        assert_eq!(output, Ok(()));
        assert_eq!(take_chunks(), &[format!("[{}:{}]\n", file, line)]);

        let (output, line) = (try_dbg!("first"), ::core::line!());
        assert_eq!(output, Ok("first"));
        assert_eq!(
            take_chunks(),
            &[format!("[{}:{}] \"first\" = \"first\"\n", file, line)]
        );

        let (output, line) = (try_dbg!(second_var), ::core::line!());
        assert_eq!(output, Ok("second"));
        assert_eq!(
            take_chunks(),
            &[format!("[{}:{}] second_var = \"second\"\n", file, line)]
        );

        let (output, line) = (try_dbg!("first", second_var), ::core::line!());
        assert_eq!(output, Ok(("first", "second")));
        assert_eq!(
            take_chunks(),
            &[
                format!("[{}:{}] \"first\" = \"first\"\n", file, line),
                format!("[{}:{}] second_var = \"second\"\n", file, line),
            ]
        );

        let second_var = second_var;
        let (output, line) = (try_dbg!(("first", second_var)), ::core::line!());
        assert_eq!(output, Ok(("first", "second")));
        assert_eq!(
            take_chunks(),
            &[format!(
                "[{}:{}] (\"first\", second_var) = (\n    \"first\",\n    \"second\",\n)\n",
                file, line
            ),]
        );
    }
}
