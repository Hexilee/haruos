use crate::serial_println;

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests:", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(exit_code as u32);
    }
}
