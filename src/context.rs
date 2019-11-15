use riscv::register::{scause::Scause, sstatus::Sstatus};

#[repr(C)]
#[derive(Debug)]
pub struct TrapFlame {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub spec: usize,
    pub stval: usize,
    pub scause: Scause,
}

impl TrapFlame {
    pub fn increase_sepc(&mut self) {
        self.spec += 4;
    }
}
