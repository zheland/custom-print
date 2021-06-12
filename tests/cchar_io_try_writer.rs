#![cfg(feature = "std")]
#![no_std]

extern crate std;

use std::os::raw::c_char;

fn write_fn(_: *const c_char) {}

custom_print::define_macros!({ println, try_println }, io, crate::write_fn);

pub mod submodule {
    #[test]
    #[should_panic(expected = "nul byte found in provided data at position: 5")]
    fn test_cchar_io_writer_failed() {
        println!("first\0second");
        unreachable!();
    }

    #[test]
    fn test_cchar_io_try_writer_failed() {
        use std::string::ToString;
        assert_eq!(try_println!("first").ok().unwrap(), ());
        assert_eq!(
            try_println!("first\0second").err().unwrap().to_string(),
            "nul byte found in provided data at position: 5"
        );
        assert_eq!(try_println!("first,second").ok().unwrap(), ());
        assert_eq!(
            try_println!("\0").err().unwrap().to_string(),
            "nul byte found in provided data at position: 0"
        );
        assert_eq!(try_println!("").ok().unwrap(), ());
    }
}
