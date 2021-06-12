#![cfg(feature = "alloc")]
#![no_std]

extern crate std;

use std::sync::atomic::{AtomicBool, Ordering};

static SHOULD_WRITE_SUCCEED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug, Eq, PartialEq)]
enum WriteError {
    CustomWriteError,
}

fn write(_: &str) -> Result<(), WriteError> {
    let should_write_succeed = SHOULD_WRITE_SUCCEED.load(Ordering::Relaxed);
    if should_write_succeed {
        Ok(())
    } else {
        Err(WriteError::CustomWriteError)
    }
}

custom_print::define_macros!({ try_print, try_println, try_dbg }, concat, crate::write);

pub mod submodule {
    #[test]
    fn test_string_writer() {
        use crate::{WriteError, SHOULD_WRITE_SUCCEED};
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
        assert_eq!(result, Err(WriteError::CustomWriteError));
        let result = try_println!("second");
        assert_eq!(result, Err(WriteError::CustomWriteError));
        let result = try_dbg!("third");
        assert_eq!(result, Err(WriteError::CustomWriteError));
    }
}
