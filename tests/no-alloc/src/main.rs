#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

custom_print::define_macros!({ print, println, dbg }, fmt, |_value: &str| { /* ... */ });

#[no_mangle]
pub extern "C" fn test() {
    use custom_print::FmtWriter;
    let _ = FmtWriter::from_closure(|_: *const u8, _: usize| {});
    print!("test");
    println!("test");
    dbg!("test");
}
