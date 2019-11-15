use crate::clock::{clock_set_next_event, TICK};
use crate::context::TrapFlame;
use crate::println;
use riscv::register::scause::{Exception, Interrupt, Trap};
use riscv::register::{scause, sepc, sscratch, sstatus, stvec};

global_asm!(include_str!("trap/trap.asm"));

#[inline(always)]
pub fn init() {
    extern "C" {
        fn __alltraps();
    }

    unsafe {
        sscratch::write(0);
        sstatus::set_sie();
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("+++++++++ setup interrupt handler ++++++++++")
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFlame) {
    //    println!("trap!");
    //    tf.increase_sepc();
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => panic!("unexpected trap"),
    }
}

fn breakpoint() {
    panic!("a breakpoint set by kernel");
}

fn super_timer() {
    clock_set_next_event();
    unsafe {
        TICK += 1;
        if TICK % 100 == 0 {
            println!("100 TICKS!");
        }
    }
}
