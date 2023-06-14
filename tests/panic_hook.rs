#![cfg(feature = "std")]
#![no_std]

extern crate std;

use std::string::String;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static MESSAGE: Lazy<Mutex<String>> = Lazy::new(Mutex::default);

fn take_message() -> String {
    use core::mem::take;
    take(&mut MESSAGE.lock().unwrap())
}

fn write(value: &str) {
    let mut chunks = MESSAGE.lock().unwrap();
    *chunks += value;
}

custom_print::define_init_panic_hook!(concat, crate::write);

mod submodule {
    use crate::{init_panic_hook, take_message};
    use std::panic::{catch_unwind, take_hook};
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_panic_hook() {
        use std::format;

        let file = file!();
        let line = AtomicU32::new(0);
        init_panic_hook();

        let result = catch_unwind(|| {
            line.store(line!() + 1, Ordering::Relaxed);
            assert_eq!(1, 2);
        });
        let _ = take_hook();
        assert!(result.is_err());
        let message = take_message();
        assert!(message.contains("panicked"));
        assert!(message.contains("assertion failed"));
        assert!(message.contains(&format!("{}:{}:", file, line.load(Ordering::Relaxed))));
    }
}
