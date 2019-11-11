use crate::println;
use riscv::register::{scause, sepc, sscratch, stvec};

#[inline(always)]
pub fn init() {
    unsafe {
        sscratch::write(0);
        stvec::write(trap_handler as usize, stvec::TrapMode::Direct);
    }
    println!("+++++++++ setup interrupt handler ++++++++++")
}

pub fn trap_handler() -> ! {
    let cause = scause::read().cause();
    let epc = sepc::read();
    println!("trap. cause: {:?}, epc: {:#x}", cause, epc);
    panic!("trap")
}
