pub mod buffer;
pub mod parser;
pub mod screen;

pub use buffer::Buffer;
pub use screen::Screen;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    LTR,
    RTL,
    Auto,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Auto
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: u8,
    pub bg: u8,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub reverse: bool,
    pub strikethrough: bool,
    pub direction: Direction,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            ch: ' ',
            fg: 7,
            bg: 0,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
            reverse: false,
            strikethrough: false,
            direction: Direction::LTR,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub visible: bool,
    pub blink: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            x: 0,
            y: 0,
            visible: true,
            blink: true,
        }
    }
}
