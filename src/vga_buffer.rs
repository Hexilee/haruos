use core::fmt::{self, Arguments, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[cfg(test)]
use crate::test_framework::*;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        row: 0,
        char_attr: CharAttribute::new(Color::LightGreen, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

const BUFFER_HEIGHT: usize = 25;
const HEIGHT_INDEX_MAX: usize = BUFFER_HEIGHT - 1;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct CharAttribute(u8);

impl CharAttribute {
    const fn new(foreground: Color, background: Color) -> CharAttribute {
        CharAttribute((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    char_attr: CharAttribute,
}

#[repr(transparent)]
struct Buffer([[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]);

pub struct Writer {
    column: usize,
    row: usize,
    char_attr: CharAttribute,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = self.row;
                let column = self.column;
                let char_attr = self.char_attr;
                self.buffer.0[row][column].write(ScreenChar {
                    ascii_character: b,
                    char_attr,
                });
                self.column += 1;
            }
        }
    }
    fn new_line(&mut self) {
        if self.row >= HEIGHT_INDEX_MAX {
            self.clear();
        } else {
            self.row += 1;
        }
        self.column = 0;
    }
    fn clear(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            char_attr: CharAttribute(0),
        };
        for row in 0..HEIGHT_INDEX_MAX {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.0[row + 1][col].read();
                self.buffer.0[row][col].write(char)
            }
        }
        for i in 0..BUFFER_WIDTH {
            self.buffer.0[HEIGHT_INDEX_MAX][i].write(blank)
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
        Ok(())
    }
}

pub fn _print(args: Arguments) -> fmt::Result {
    WRITER.lock().write_fmt(args)
}

pub macro print($($args: tt)*) {
    _print(format_args!($($args)*)).expect("fail to print")
}

pub macro println($($args:tt)*) {
    print!("{}\n", format_args!($($args)*))
}

#[test_case]
fn test_print_simple() {
    serial_print!("test_print_simple... ");
    println!("simple output");
    serial_println!("[ok]");
}
