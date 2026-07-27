use super::{Buffer, Cell, Direction};

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParserState {
    Ground,
    Escape,
    Csi,
    Osc,
}

#[derive(Debug, Clone)]
pub struct Parser {
    state: ParserState,
    params: Vec<u16>,
    current_param: String,
    /// Current SGR attributes active for new characters
    pub fg: u8,
    pub bg: u8,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub reverse: bool,
    pub strikethrough: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: ParserState::Ground,
            params: Vec::new(),
            current_param: String::new(),
            fg: 7,
            bg: 0,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
            reverse: false,
            strikethrough: false,
        }
    }

    fn apply_sgr(&mut self, params: &[u16]) {
        if params.is_empty() {
            self.reset_attrs();
            return;
        }
        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => self.reset_attrs(),
                1 => self.bold = true,
                2 => self.dim = true,
                3 => self.italic = true,
                4 => self.underline = true,
                7 => self.reverse = true,
                9 => self.strikethrough = true,
                22 => { self.bold = false; self.dim = false; }
                23 => self.italic = false,
                24 => self.underline = false,
                27 => self.reverse = false,
                29 => self.strikethrough = false,
                30..=37 => self.fg = (params[i] - 30) as u8,
                39 => self.fg = 7,
                40..=47 => self.bg = (params[i] - 40) as u8,
                49 => self.bg = 0,
                90..=97 => self.fg = (params[i] - 90 + 8) as u8,
                100..=107 => self.bg = (params[i] - 100 + 8) as u8,
                _ => {}
            }
            i += 1;
        }
    }

    fn reset_attrs(&mut self) {
        self.fg = 7;
        self.bg = 0;
        self.bold = false;
        self.italic = false;
        self.underline = false;
        self.dim = false;
        self.reverse = false;
        self.strikethrough = false;
    }

    /// Push the final param from current_param into params (for CSI sequences)
    fn push_final_param(&mut self) {
        if !self.current_param.is_empty() {
            let param: u16 = self.current_param.parse().unwrap_or(0);
            self.params.push(param);
            self.current_param.clear();
        }
    }

    /// Create a Cell with the current SGR attributes
    fn make_cell(&self, ch: char) -> Cell {
        Cell {
            ch,
            fg: self.fg,
            bg: self.bg,
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            dim: self.dim,
            reverse: self.reverse,
            strikethrough: self.strikethrough,
            direction: Direction::LTR,
        }
    }

    pub fn parse(&mut self, buffer: &mut Buffer, input: &str, cursor_x: &mut u16, cursor_y: &mut u16) {
        for ch in input.chars() {
            match self.state {
                ParserState::Ground => {
                    match ch {
                        '\x1b' => {
                            self.state = ParserState::Escape;
                            self.params.clear();
                            self.current_param.clear();
                        }
                        '\r' => {
                            *cursor_x = 0;
                        }
                        '\n' => {
                            *cursor_y = (*cursor_y + 1).min(buffer.rows - 1);
                            if *cursor_y == buffer.rows - 1 {
                                buffer.scroll_up(1);
                                *cursor_y = buffer.rows - 1;
                            }
                        }
                        '\t' => {
                            let tab_stop = (*cursor_x / 8 + 1) * 8;
                            *cursor_x = tab_stop.min(buffer.cols - 1);
                        }
                        '\x08' => {
                            if *cursor_x > 0 {
                                *cursor_x -= 1;
                            }
                        }
                        '\x07' => {
                            // Bell
                        }
                        ch if ch.is_control() => {
                            // Other control characters
                        }
                        ch => {
                            let cell = self.make_cell(ch);
                            let actual_y = buffer.actual_y(*cursor_y as usize);
                            buffer.set_cell(*cursor_x as usize, actual_y, cell);
                            *cursor_x += 1;
                            if *cursor_x >= buffer.cols {
                                *cursor_x = 0;
                                *cursor_y += 1;
                                if *cursor_y >= buffer.rows {
                                    buffer.scroll_up(1);
                                    *cursor_y = buffer.rows - 1;
                                }
                            }
                        }
                    }
                }
                ParserState::Escape => {
                    match ch {
                        '[' => {
                            self.state = ParserState::Csi;
                            self.params.clear();
                            self.current_param.clear();
                        }
                        ']' => {
                            self.state = ParserState::Osc;
                        }
                        'M' => {
                            if *cursor_y > 0 {
                                *cursor_y -= 1;
                            }
                            self.state = ParserState::Ground;
                        }
                        'c' => {
                            buffer.clear();
                            *cursor_x = 0;
                            *cursor_y = 0;
                            self.state = ParserState::Ground;
                        }
                        _ => {
                            self.state = ParserState::Ground;
                        }
                    }
                }
                ParserState::Csi => {
                    match ch {
                        '0'..='9' => {
                            self.current_param.push(ch);
                        }
                        ';' => {
                            let param: u16 = self.current_param.parse().unwrap_or(0);
                            self.params.push(param);
                            self.current_param.clear();
                        }
                        'A' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            *cursor_y = cursor_y.saturating_sub(param);
                            self.state = ParserState::Ground;
                        }
                        'B' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            *cursor_y = (*cursor_y + param).min(buffer.rows - 1);
                            self.state = ParserState::Ground;
                        }
                        'C' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            *cursor_x = (*cursor_x + param).min(buffer.cols - 1);
                            self.state = ParserState::Ground;
                        }
                        'D' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            *cursor_x = cursor_x.saturating_sub(param);
                            self.state = ParserState::Ground;
                        }
                        'E' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            *cursor_x = 0;
                            *cursor_y = (*cursor_y + param).min(buffer.rows - 1);
                            self.state = ParserState::Ground;
                        }
                        'F' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            *cursor_x = 0;
                            *cursor_y = cursor_y.saturating_sub(param);
                            self.state = ParserState::Ground;
                        }
                        'G' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            *cursor_x = param.saturating_sub(1).min(buffer.cols - 1);
                            self.state = ParserState::Ground;
                        }
                        'H' | 'f' => {
                            self.push_final_param();
                            let row = self.params.first().copied().unwrap_or(1).saturating_sub(1);
                            let col = self.params.get(1).copied().unwrap_or(1).saturating_sub(1);
                            *cursor_y = row.min(buffer.rows - 1);
                            *cursor_x = col.min(buffer.cols - 1);
                            self.state = ParserState::Ground;
                        }
                        'J' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(0);
                            match param {
                                0 => {
                                    if let Some(line) = buffer.get_line_mut(buffer.actual_y(*cursor_y as usize)) {
                                        for x in *cursor_x as usize..line.cells.len() {
                                            line.cells[x] = Cell::default();
                                        }
                                    }
                                    for y in (*cursor_y as usize + 1)..buffer.rows as usize {
                                        let actual_y = buffer.actual_y(y);
                                        buffer.clear_line(actual_y);
                                    }
                                }
                                1 => {
                                    for y in 0..*cursor_y as usize {
                                        let actual_y = buffer.actual_y(y);
                                        buffer.clear_line(actual_y);
                                    }
                                    if let Some(line) = buffer.get_line_mut(buffer.actual_y(*cursor_y as usize)) {
                                        for x in 0..=*cursor_x as usize {
                                            line.cells[x] = Cell::default();
                                        }
                                    }
                                }
                                2 => {
                                    for y in 0..buffer.rows as usize {
                                        let actual_y = buffer.actual_y(y);
                                        buffer.clear_line(actual_y);
                                    }
                                }
                                3 => {
                                    // Clear scrollback
                                    let viewport_top = buffer.lines.len().saturating_sub(buffer.rows as usize);
                                    if viewport_top > 0 {
                                        buffer.lines.drain(0..viewport_top);
                                    }
                                }
                                _ => {}
                            }
                            self.state = ParserState::Ground;
                        }
                        'K' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(0);
                            let actual_y = buffer.actual_y(*cursor_y as usize);
                            match param {
                                0 => {
                                    if let Some(line) = buffer.get_line_mut(actual_y) {
                                        for x in *cursor_x as usize..line.cells.len() {
                                            line.cells[x] = Cell::default();
                                        }
                                    }
                                }
                                1 => {
                                    if let Some(line) = buffer.get_line_mut(actual_y) {
                                        for x in 0..=*cursor_x as usize {
                                            line.cells[x] = Cell::default();
                                        }
                                    }
                                }
                                2 => {
                                    buffer.clear_line(actual_y);
                                }
                                _ => {}
                            }
                            self.state = ParserState::Ground;
                        }
                        'L' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            let cols = buffer.cols;
                            let actual_y = buffer.actual_y(*cursor_y as usize);
                            let bottom_idx = buffer.actual_y(buffer.rows as usize - 1);
                            for _ in 0..param {
                                if actual_y <= bottom_idx {
                                    let new_line = super::buffer::Line::new(cols);
                                    buffer.lines.insert(actual_y, new_line);
                                    if bottom_idx + 1 < buffer.lines.len() {
                                        buffer.lines.remove(bottom_idx + 1);
                                    }
                                }
                            }
                            self.state = ParserState::Ground;
                        }
                        'M' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            let cols = buffer.cols;
                            let actual_y = buffer.actual_y(*cursor_y as usize);
                            let bottom_idx = buffer.actual_y(buffer.rows as usize - 1);
                            for _ in 0..param {
                                if actual_y <= bottom_idx && actual_y < buffer.lines.len() {
                                    buffer.lines.remove(actual_y);
                                    buffer.lines.insert(bottom_idx, super::buffer::Line::new(cols));
                                }
                            }
                            self.state = ParserState::Ground;
                        }
                        'P' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            let cols = buffer.cols as usize;
                            let actual_y = buffer.actual_y(*cursor_y as usize);
                            if let Some(line) = buffer.get_line_mut(actual_y) {
                                let start = *cursor_x as usize;
                                let end = (start + param as usize).min(line.cells.len());
                                line.cells.drain(start..end);
                                while line.cells.len() < cols {
                                    line.cells.push(Cell::default());
                                }
                            }
                            self.state = ParserState::Ground;
                        }
                        'S' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            buffer.scroll_up(param as usize);
                            self.state = ParserState::Ground;
                        }
                        'T' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            let cols = buffer.cols;
                            let top_y = buffer.actual_y(0);
                            let bottom_idx = buffer.actual_y(buffer.rows as usize - 1);
                            for _ in 0..param {
                                buffer.lines.insert(top_y, super::buffer::Line::new(cols));
                                if bottom_idx + 1 < buffer.lines.len() {
                                    buffer.lines.remove(bottom_idx + 1);
                                }
                            }
                            self.state = ParserState::Ground;
                        }
                        'X' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            let actual_y = buffer.actual_y(*cursor_y as usize);
                            if let Some(line) = buffer.get_line_mut(actual_y) {
                                let start = *cursor_x as usize;
                                let end = (start + param as usize).min(line.cells.len());
                                for cell in &mut line.cells[start..end] {
                                    *cell = Cell::default();
                                }
                            }
                            self.state = ParserState::Ground;
                        }
                        '@' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            let actual_y = buffer.actual_y(*cursor_y as usize);
                            if let Some(line) = buffer.get_line_mut(actual_y) {
                                let start = *cursor_x as usize;
                                let insert_count = param as usize;
                                for i in (start..line.cells.len() - insert_count).rev() {
                                    line.cells[i + insert_count] = line.cells[i];
                                }
                                for cell in &mut line.cells[start..start + insert_count] {
                                    *cell = Cell::default();
                                }
                            }
                            self.state = ParserState::Ground;
                        }
                        'd' => {
                            self.push_final_param();
                            let param = self.params.first().copied().unwrap_or(1);
                            *cursor_y = param.saturating_sub(1).min(buffer.rows - 1);
                            self.state = ParserState::Ground;
                        }
                        'm' => {
                            self.push_final_param();
                            let params = self.params.clone();
                            self.apply_sgr(&params);
                            self.state = ParserState::Ground;
                        }
                        'n' | 'r' | 's' | 'u' | 'h' | 'l' => {
                            self.current_param.clear();
                            self.params.clear();
                            self.state = ParserState::Ground;
                        }
                        '?' => {
                            self.current_param.clear();
                        }
                        _ => {
                            self.state = ParserState::Ground;
                        }
                    }
                }
                ParserState::Osc => {
                    if ch == '\x07' {
                        self.state = ParserState::Ground;
                    }
                }
            }
        }
    }
}
