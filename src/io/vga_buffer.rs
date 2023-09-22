//! The quickest way to write to a screen is to directly
//! use the VGA buffer. It lives on 0xb8000 address. Now,
//! there are more ways to render stuff on screen but VGA
//! is the most commanly used one & it's simple.
//!
//! The VGA buffer consists of 25 rows & 80 columns. Think of
//! it as an array that you can play around with. Now, you can
//! put your character in the array & get stuff to show on
//! screen.
//!
//!
//! - Each charcter in the VGA Buffer has the following structure.
//!     |........|...x|...y|   x = bright bit, y = blink bit
//!       code     bg   fg
//!       point   color color
//!
//! Remember, In the below bytes bits start from 0 not 1.
//!
//! - Each character is of two bytes. The first byte is designated for the
//! codepage 437 character code point. The second byte is designated
//! for the color information.
//! - In the second byte, the first 3 bits are designated for the foreground
//! color, the next bit is designated for the bright bit. The next 3 bits
//! are designated for the background color. The last bit is designated for the
//! blink bit.
//!
//! The bright bit turns the color to a lighter tone.

use core::{usize, fmt};
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

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
    White = 15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// We perform the leftshit operation to make space for the
    /// foreground color. You may wonder, why is background first &
    /// foreground last? It's because we're using Little Endian. So
    /// we store the least significant byte first.
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

/// The VGA buffer.
#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

/// The writer will write stuff to the buffer.
pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer
}

impl Writer {
    /// Writes a byte onto the buffer.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                // if exceeded line capacity, change line.
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                // write the character along with it's color.
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                // advance column position by 1.
                self.column_position += 1;
            }
        }
    }

    /// Print a new line. This involves shifting each character up by 1 row
    /// in the buffer. Then we clear the first row with blank characters.
    fn new_line(&mut self) {
        // iterate over each character and shift them up by 1 line.
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Write blank character's onto the buffer row.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// Only write the bytes that are supported. If unsupported, print a blockish character.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Now we use the same std lib print macros but instead call our writer
/// to print the text to the VGA buffer.

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
