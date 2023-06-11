#![cfg(feature = "std")]
#![no_std]

extern crate std;

use core::str::from_utf8;
use std::io::{self, BufWriter, Write};
use std::string::String;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static WRITTEN: Lazy<Mutex<String>> = Lazy::new(Mutex::default);
static LINE_STDOUT: Lazy<Mutex<BufWriter<ChunkWriter>>> =
    Lazy::new(|| Mutex::new(BufWriter::new(ChunkWriter::default())));

#[derive(Clone, Debug, Default)]
struct ChunkWriter;

impl Write for ChunkWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = WRITTEN.lock().unwrap();
        *written += from_utf8(buf).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn get_written() -> String {
    use std::string::ToString;
    WRITTEN.lock().unwrap().to_string()
}

fn get_buffered() -> String {
    use std::string::ToString;
    from_utf8(LINE_STDOUT.lock().unwrap().buffer())
        .unwrap()
        .to_string()
}

fn clear_written() {
    let mut written = WRITTEN.lock().unwrap();
    written.clear();
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
    &mut *LINE_STDOUT.lock().unwrap() as &mut dyn ::std::io::Write
);

pub mod submodule {
    #[test]
    fn test_buf_writer() {
        use crate::{black_box, clear_written, get_buffered, get_written, LINE_STDOUT};
        use core::iter::repeat;
        use std::string::String;

        print!("first");
        assert_eq!(get_written(), "");
        assert_eq!(get_buffered(), "first");
        print!("first {}\nthird", black_box("second"));
        assert_eq!(get_written(), "");
        assert_eq!(get_buffered(), "firstfirst second\nthird");
        println!();
        assert_eq!(get_written(), "");
        assert_eq!(get_buffered(), "firstfirst second\nthird\n");

        flush!();
        assert_eq!(get_written(), "firstfirst second\nthird\n");
        assert_eq!(get_buffered(), "");
        clear_written();

        let capacity = LINE_STDOUT.lock().unwrap().capacity();
        let data = repeat("\n").take(capacity / 2 + 1).collect::<String>();
        print!("{}", data);
        let written = get_written();
        assert_eq!(written.len(), 0);
        print!("{}", data);
        let written = get_written();
        assert_ne!(written.len(), 0);
    }
}
