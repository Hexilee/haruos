#![no_std]
#![feature(lang_items, start)]

//extern crate libc;

use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort")
}

#[start]
fn start(_argc: isize, _args: *const *const u8) -> isize {
    loop {}
}
