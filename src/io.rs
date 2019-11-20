use crate::vga_buffer::WRITER;
use core::fmt::{self, Arguments, Write};

pub fn print(args: Arguments) -> fmt::Result {
    WRITER.write_fmt(args)
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
        crate::io::putchar('\n')
    };

    ($($args:tt)*) => {
        crate::print!("{}\n", format_args!($($args)*))
    };
}
