#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(
    lang_items,
    asm,
    global_asm,
    alloc_error_handler,
    custom_test_frameworks
)]
#![test_runner(test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod clock;
mod context;
mod init;
mod interrupt;
mod io;
mod memory;
mod panic;
mod serial;
mod test_framework;
mod vga_buffer;

#[cfg(test)]
#[no_mangle]
extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
extern "C" fn _start() -> ! {
    println!("kernel boot");
    loop {}
}
