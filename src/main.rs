#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(lang_items, asm, global_asm, alloc_error_handler)]

mod clock;
mod context;
mod init;
mod interrupt;
mod io;
mod lang_items;
mod memory;
mod sbi;
