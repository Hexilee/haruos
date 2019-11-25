#[cfg(not(test))]
use crate::println;
#[cfg(test)]
use crate::test_framework::*;
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
