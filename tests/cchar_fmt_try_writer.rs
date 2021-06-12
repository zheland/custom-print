#![cfg(feature = "std")]
#![no_std]

extern crate std;

use std::os::raw::c_char;

fn write_fn(_: *const c_char) {}

custom_print::define_macros!({ println, try_println }, fmt, crate::write_fn);

pub mod submodule {
    #[test]
    #[should_panic(expected = "nul byte found in provided data at position: 5")]
    fn test_cchar_fmt_writer_failed() {
        println!("first\0second");
        unreachable!();
    }

    #[test]
    fn test_cchar_fmt_try_writer_failed() {
        assert_eq!(try_println!("first"), Ok(()));
        assert_eq!(try_println!("first\0second"), Err(core::fmt::Error));
        assert_eq!(try_println!("first,second"), Ok(()));
        assert_eq!(try_println!("\0"), Err(core::fmt::Error));
        assert_eq!(try_println!(""), Ok(()));
    }
}
