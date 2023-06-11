#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use core::ptr::null_mut;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static GLOBAL_ALLOCATOR: DummyAllocator = DummyAllocator;

custom_print::define_macros!({ print, println, dbg }, concat, |_value: &str| { /* ... */ });

#[no_mangle]
pub extern "C" fn test() {
    use custom_print::FmtWriter;
    let _ = FmtWriter::from_closure(|_: *const u8, _: usize| {});
    use custom_print::ConcatWriter;
    let _ = ConcatWriter::from_closure(|_: *const u8, _: usize| {});
    print!("test");
    println!("test");
    dbg!("test");
}
