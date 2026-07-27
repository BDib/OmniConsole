use super::{Buffer, Cursor, parser::Parser};

#[derive(Debug, Clone)]
pub struct Screen {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub parser: Parser,
    pub rtl_mode: bool,
}

impl Screen {
    pub fn new(cols: u16, rows: u16, scrollback: usize) -> Self {
        Screen {
            buffer: Buffer::new(cols, rows, scrollback),
            cursor: Cursor::default(),
            parser: Parser::new(),
            rtl_mode: false,
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.buffer.resize(cols, rows);
        self.cursor.x = self.cursor.x.min(cols - 1);
        self.cursor.y = self.cursor.y.min(rows - 1);
    }

    pub fn process_input(&mut self, input: &str) {
        self.parser.parse(
            &mut self.buffer,
            input,
            &mut self.cursor.x,
            &mut self.cursor.y,
        );
        self.buffer.detect_directions();
    }

    pub fn set_rtl_mode(&mut self, rtl: bool) {
        self.rtl_mode = rtl;
    }
}
