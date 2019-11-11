use crate::println;

global_asm!(include_str!("boot/entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    println!("hello, {}", "world!");
    loop {}
}
