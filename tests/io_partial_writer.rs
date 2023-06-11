#![cfg(feature = "std")]
#![no_std]

extern crate std;

use core::str::from_utf8;
use std::string::String;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::vec::Vec;

use once_cell::sync::Lazy;

static CHUNKS: Lazy<Mutex<Vec<String>>> = Lazy::new(Mutex::default);
static MAX_CHUNK_LEN: AtomicUsize = AtomicUsize::new(0);

fn take_chunks() -> Vec<String> {
    use core::mem::take;
    take(&mut CHUNKS.lock().unwrap())
}

fn write_fn(value: &[u8]) -> usize {
    use std::string::ToString;
    let mut chunks = CHUNKS.lock().unwrap();
    chunks.push(from_utf8(value).unwrap().to_string());
    value.len().min(MAX_CHUNK_LEN.load(Ordering::Relaxed))
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
    fn test_io_partial_writer() {
        use crate::{black_box, take_chunks, MAX_CHUNK_LEN};
        use std::sync::atomic::Ordering;

        MAX_CHUNK_LEN.store(4, Ordering::Relaxed);
        print!("first");
        assert_eq!(take_chunks(), &["first", "t"]);

        MAX_CHUNK_LEN.store(2, Ordering::Relaxed);
        print!("first {}\nthird\n", black_box("second"));
        let chunks = take_chunks();
        if chunks[0] == "first " {
            // rust version <= 1.70
            assert_eq!(
                chunks,
                [
                    "first ",
                    "rst ",
                    "t ",
                    "second",
                    "cond",
                    "nd",
                    "\nthird\n",
                    "hird\n",
                    "rd\n",
                    "\n"
                ]
            );
        } else {
            // rust version >= 1.71
            assert_eq!(
                chunks,
                [
                    "first second\nthird\n",
                    "rst second\nthird\n",
                    "t second\nthird\n",
                    "second\nthird\n",
                    "cond\nthird\n",
                    "nd\nthird\n",
                    "\nthird\n",
                    "hird\n",
                    "rd\n",
                    "\n"
                ]
            );
        }

        println!();
        assert_eq!(take_chunks(), &["\n"]);
    }
}
