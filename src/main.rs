#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(lang_items, asm, global_asm, alloc_error_handler)]
mod clock;
mod context;
mod init;
mod interrupt;
mod io;
mod memory;
mod panic;

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello, {}", "World");
    loop {}
}
