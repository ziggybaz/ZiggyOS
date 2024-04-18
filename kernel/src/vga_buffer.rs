//vga_text buffer
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
    colour_code: Colour_Code,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)] //ensures same memory layout as its single field
struct Buffer {
    characters: [[ScreenCharacter; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//writing to screen
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}








































