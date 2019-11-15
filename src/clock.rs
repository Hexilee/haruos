use crate::println;
use crate::sbi::set_timer;
use riscv::register::{sie, time, timeh};

pub static mut TICK: usize = 0;
const TIME_BASE: u64 = 100000;
pub fn init() {
    unsafe {
        TICK = 0;
        sie::set_stimer();
    }
    clock_set_next_event();
    println!("++++++ set timer! ++++++");
}

pub fn clock_set_next_event() {
    set_timer(get_cycle() + TIME_BASE);
}

fn get_cycle() -> u64 {
    loop {
        let hi = timeh::read();
        let lo = time::read();
        if hi == timeh::read() {
            break ((hi as u64) << 32) | (lo as u64);
        }
    }
}
