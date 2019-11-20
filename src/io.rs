use core::fmt::{self, Arguments, Write};

struct Stdout;

pub fn puts(s: &str) {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, byte) in s.bytes().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Ok(puts(s))
    }
}

pub fn print(args: Arguments) -> fmt::Result {
    Stdout.write_fmt(args)
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
