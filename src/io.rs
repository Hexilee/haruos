use crate::serial::SERIAL_0;
use crate::vga_buffer::WRITER;
use core::fmt::{self, Arguments, Write};

pub fn print(args: Arguments) -> fmt::Result {
    WRITER.lock().write_fmt(args)
}

pub fn serial_print(args: Arguments) -> fmt::Result {
    SERIAL_0.lock().write_fmt(args)
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        crate::io::print(format_args!($($args)*)).expect("fail to print")
    };
}

#[macro_export]
macro_rules! println {
    () => {
        crate::print!("\n")
    };

    ($($args:tt)*) => {
        crate::print!("{}\n", format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! serial_print {
    ($($args:tt)*) => {
        crate::io::serial_print(format_args!($($args)*)).expect("fail to serial print")
    };
}

#[macro_export]
macro_rules! serial_println {
    () => {
        crate::serial_print("\n")
    };

    ($($args:tt)*) => {
        crate::serial_print!("{}\n", format_args!($($args)*))
    };
}
