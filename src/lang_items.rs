use crate::println;
use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort")
}
