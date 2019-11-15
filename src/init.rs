use crate::println;
use riscv::asm::ebreak;

global_asm!(include_str!("boot/entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    crate::interrupt::init();
    crate::clock::init();
    unsafe {
        ebreak();
    }
    loop {}
}
