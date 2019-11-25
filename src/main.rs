#![no_std] // don't link the Rust standard library
#![no_main]
#![feature(lang_items, custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod init;
mod interrupt;
mod io;
mod memory;
mod panic;
mod serial;

#[cfg(test)]
mod test_framework;
mod vga_buffer;

#[cfg(test)]
#[no_mangle]
extern "C" fn _start() -> ! {
    init::init();
    test_main();
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
extern "C" fn _start() -> ! {
    println!("kernel boot");
    init::init();
    x86_64::instructions::interrupts::int3();
    println!("boot continue");
    loop {}
}
