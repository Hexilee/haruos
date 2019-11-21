use crate::println;
use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    println!("kernel panic: {}", info);
    loop {}
}
