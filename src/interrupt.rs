use crate::context::TrapFlame;
use crate::println;
use riscv::register::{scause, sepc, sscratch, stvec};

global_asm!(include_str!("trap/trap.asm"));

#[inline(always)]
pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe {
        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("+++++++++ setup interrupt handler ++++++++++")
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFlame) {
    println!("trap!");
    tf.increase_sepc();
}
