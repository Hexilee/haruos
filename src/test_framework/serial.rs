use core::fmt::{self, Arguments, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL_0: Mutex<SerialPort> = {
        let mut port = unsafe { SerialPort::new(0x3F8) };
        port.init();
        Mutex::new(port)
    };
}

pub fn _print(args: Arguments) -> fmt::Result {
    SERIAL_0.lock().write_fmt(args)
}

pub macro print($($args: tt)*) {
    _print(format_args!($($args)*)).expect("fail to serial print")
}

pub macro println($($args:tt)*) {
    print!("{}\n", format_args!($($args)*))
}
