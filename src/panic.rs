use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    loop {}
}
