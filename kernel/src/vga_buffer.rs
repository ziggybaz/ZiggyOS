//vga_text buffer
use volatile::Volatile; //allows you to manage aggressive Rust compiler optimizations, as we're only writing to buffer and never reading from it, the compiler doesn't know we're using VGA Buffer memory so ight decide that
                        //these writes are unneccesary and omit them therefore we to avoid this we need to specify these writes as VOLATILE.
use core::fmt;  //introducing Rust's formatting macros
use core::fmt::Write;

//a. colour
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour { //'C-like enum' to explicitly specify the number for each colour.
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8)) //bitwise operation for memory
    }
}

//b.text buffer
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)] //essential for field ordering as Rust doesn't mind field ordering and we are therefore to use C like rules for this struct which does value field-ordering.
struct ScreenCharacter {
    ascii_character: u8,
    colour_code: ColourCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

//stores a reference to the VGA buffer
#[repr(transparent)] //ensures same memory layout as its single field
struct Buffer {
    characters: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//writing to screen/modifying buffer characters
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

#[allow(unused_variables)]
//printing(we'll use the Writer to modify the buffer's characters)
impl Writer {
    //writing a single ascii byte.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH { self.new_line(); }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let colour_code = self.colour_code;
                self.buffer.characters[row][col].write( ScreenCharacter {  //guarantees the compiler will never optimize away this write as it's marked as volatile.
                    ascii_character: byte,
                    colour_code,
                });
                self.column_position += 1;
            }
        }
    }

    //converting strings to bytes and printing them out one by one
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), //vga text buffer only supports ascii, Rust strings are utf-8 by default so might contain unsupported bytes, this here differentiates printable & unprintable
                _ => self.write_byte(0xfe), //for unprinable, we print a character of '0xfe' hex code on the vga hardware
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {  //iterating over all screen characters and move each character one row up
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.characters[row][col].read();
                self.buffer.characters[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT -1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) { //this method clears a row by overwriting all of its characters with a space character
        let blank = ScreenCharacter {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.characters[row][col].write(blank)
        }
    }
}

//introducing formatting macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

//let's see if it works, wowza.
pub fn print_to_screen() {
    let mut writer = Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Green, Colour::Red),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'Z');
    writer.write_string("iggyOS ");
    writer.write_string("is a learning OS for my purposes only,");
    writer.write_string(" maybe there's a trojan horse in here.");
    write!(writer, "Dummy math op {} + {} = {}", 67, 90, 67+90).expect("Don't want it to panic. In any case it won't since writes to the VGA buffer never fail");
}



































