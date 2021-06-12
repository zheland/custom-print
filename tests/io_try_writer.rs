#![cfg(feature = "std")]
#![no_std]

extern crate std;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

static SHOULD_WRITE_SUCCEED: AtomicBool = AtomicBool::new(false);

fn write(value: &[u8]) -> io::Result<usize> {
    let should_write_succeed = SHOULD_WRITE_SUCCEED.load(Ordering::Relaxed);
    if should_write_succeed {
        Ok(value.len())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
    }
}

custom_print::define_macros!({ try_print, try_println, try_dbg }, io, crate::write);

pub mod submodule {
    #[test]
    fn test_string_writer() {
        use crate::SHOULD_WRITE_SUCCEED;
        use std::sync::atomic::Ordering;

        SHOULD_WRITE_SUCCEED.store(true, Ordering::Relaxed);
        let result = try_print!("first");
        assert_eq!(result.ok(), Some(()));
        let result = try_println!("second");
        assert_eq!(result.ok(), Some(()));
        let result = try_dbg!("third");
        assert_eq!(result.ok(), Some("third"));

        SHOULD_WRITE_SUCCEED.store(false, Ordering::Relaxed);
        let result = try_print!("first");
        assert!(result.is_err());
        let result = try_println!("second");
        assert!(result.is_err());
        let result = try_dbg!("third");
        assert!(result.is_err());
    }
}
