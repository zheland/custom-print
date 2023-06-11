#![cfg(feature = "std")]
#![no_std]

extern crate std;

use core::str::from_utf8;
use std::io::{self, LineWriter, Write};
use std::string::String;
use std::sync::Mutex;
use std::vec::Vec;

use once_cell::sync::Lazy;

static CHUNKS: Lazy<Mutex<Vec<String>>> = Lazy::new(Mutex::default);
static LINE_STDOUT: Lazy<Mutex<LineWriter<ChunkWriter>>> =
    Lazy::new(|| Mutex::new(LineWriter::new(ChunkWriter)));

#[derive(Clone, Debug, Default)]
struct ChunkWriter;

impl Write for ChunkWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use std::string::ToString;
        let mut chunks = CHUNKS.lock().unwrap();
        chunks.push(from_utf8(buf).unwrap().to_string());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn take_chunks() -> Vec<String> {
    use core::mem::take;
    take(&mut CHUNKS.lock().unwrap())
}

#[inline(never)]
fn black_box<D>(input: D) -> D {
    unsafe {
        let output = std::ptr::read_volatile(&input);
        std::mem::forget(input);
        output
    }
}

custom_print::define_macros!(
    { print, println, dbg, flush },
    &mut *crate::LINE_STDOUT.lock().unwrap() as &mut dyn ::std::io::Write
);

pub mod submodule {
    #[test]
    fn test_line_writer() {
        use crate::{black_box, take_chunks};

        print!("first");
        assert_eq!(take_chunks(), &[""; 0][..]);
        print!("first {}\nthird\n", black_box("second"));
        assert_eq!(take_chunks(), &["firstfirst second\nthird\n"]);

        println!();
        assert_eq!(take_chunks(), &["\n"]);
        println!("first");
        assert_eq!(take_chunks(), &["first\n"]);
        println!("first {}\nthird\n", black_box("second"));
        assert_eq!(take_chunks(), &["first second\nthird\n\n"]);

        print!("first");
        print!("\nsecond");
        print!(" third");
        assert_eq!(take_chunks(), &["first\n"]);
        flush!();
        assert_eq!(take_chunks(), &["second third"]);
    }
}
