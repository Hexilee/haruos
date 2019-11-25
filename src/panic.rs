use crate::test_framework::{exit_qemu, QemuExitCode};
use crate::{println, serial_println};
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    println!("kernel panic: {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
