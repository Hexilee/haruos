#![no_std] // don't link the Rust standard library
#![no_main]
#![feature(lang_items, custom_test_frameworks, abi_x86_interrupt, decl_macro)]
#![test_runner(test_framework::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod gdt;
mod init;
mod interrupt;
mod panic;
mod vga_buffer;

#[cfg(test)]
mod test_framework;

pub use vga_buffer::{print, println};

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
    //    x86_64::instructions::interrupts::int3();
    fn stack_overflow() {
        stack_overflow()
    }
    stack_overflow();
    println!("boot continue");
    loop {}
}
