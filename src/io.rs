use crate::sbi;
use core::fmt::{self, Arguments, Write};

struct Stdout;

pub fn putchar(c: char) {
    sbi::console_putchar(c as usize)
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putchar(c)
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
