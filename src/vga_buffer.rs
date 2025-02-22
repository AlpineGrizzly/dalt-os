#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

// Color palette enum
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
#[repr(transparent)] // Ensures that struct has same memory layout as its single field
struct ColorCode(u8);
// Color code struct for representing the color byte that will be applied to characters
impl ColorCode { 
    fn new (foreground: Color, background: Color) -> ColorCode { 
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
// Buffer structure for representing screen characters and the text buffer
struct ScreenChar { 
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
#[repr(transparent)]
struct Buffer { 
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Writer struct that will be used for writing characters to the screen
pub struct Writer { 
    column_position: usize,      // Keeps track of current position on the last row
    color_code: ColorCode,       // Current foreground and background colors
    buffer: &'static mut Buffer, // Reference to vga buffer ('static says reference is valid for whole runtime)
}

use spin::Mutex;
use lazy_static::lazy_static; // due to non-consts, will initialized for the first time at runtime
// Static Writer that interface with other modules without a writer instance
// Initialized at compile time
lazy_static! {
    // Leverage spinlock mutex for interior mutability
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { 
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Writer {
    // Method for writing a single ascii byte 
    pub fn write_byte(&mut self, byte: u8) { 
        match byte {
            b'\n' => self.new_line(),
            byte => { 
                if self.column_position >= BUFFER_WIDTH { 
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar { 
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    
    pub fn write_string(&mut self, s: &str) { 
        for byte in s.bytes() { 
           match byte { 
            // Printable ASCII byte or newline
            0x20..=0x7e | b'\n' => self.write_byte(byte),
            // not part of printable ascii range
            _ => self.write_byte(0xfe),
           } 
        }
    }

    fn new_line(&mut self) { 
        for row in 1..BUFFER_HEIGHT { 
            for col in 0..BUFFER_WIDTH { 
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) { 
        let blank = ScreenChar { 
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH { 
            self.buffer.chars[row][col].write(blank);
        }
    }
}

use core::fmt;

impl fmt::Write for Writer { 
    fn write_str(&mut self, s: &str) -> fmt::Result { 
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
// $crate allows for the macro to work from outside the crate
// format_args builds a fmt::Arguments type from passed arguments which is passed to _print
// The below implementations simply copy the println! and print! macros but modify them
// to use the _print function for the vga_buffer
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
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
