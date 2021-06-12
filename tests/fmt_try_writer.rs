#![no_std]

extern crate std;

use core::fmt;
use core::sync::atomic::{AtomicBool, Ordering};

static SHOULD_WRITE_SUCCEED: AtomicBool = AtomicBool::new(false);

fn write(_: &str) -> fmt::Result {
    let should_write_succeed = SHOULD_WRITE_SUCCEED.load(Ordering::Relaxed);
    if should_write_succeed {
        Ok(())
    } else {
        Err(fmt::Error)
    }
}

custom_print::define_macros!({ try_print, try_println, try_dbg }, fmt, crate::write);

pub mod submodule {
    #[test]
    fn test_string_writer() {
        use crate::SHOULD_WRITE_SUCCEED;
        use core::fmt;
        use std::sync::atomic::Ordering;

        SHOULD_WRITE_SUCCEED.store(true, Ordering::Relaxed);
        let result = try_print!("first");
        assert_eq!(result, Ok(()));
        let result = try_println!("second");
        assert_eq!(result, Ok(()));
        let result = try_dbg!("third");
        assert_eq!(result, Ok("third"));

        SHOULD_WRITE_SUCCEED.store(false, Ordering::Relaxed);
        let result = try_print!("first");
        assert_eq!(result, Err(fmt::Error));
        let result = try_println!("second");
        assert_eq!(result, Err(fmt::Error));
        let result = try_dbg!("third");
        assert_eq!(result, Err(fmt::Error));
    }
}
