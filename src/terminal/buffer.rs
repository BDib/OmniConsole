use super::{Cell, Direction};
use crate::bidi::BidiAnalyzer;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub cols: u16,
    pub rows: u16,
    pub scrollback: usize,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub cells: Vec<Cell>,
    pub direction: Direction,
}

impl Buffer {
    pub fn new(cols: u16, rows: u16, scrollback: usize) -> Self {
        let mut lines = Vec::with_capacity(rows as usize + scrollback);
        for _ in 0..rows {
            lines.push(Line::new(cols));
        }
        
        Buffer {
            lines,
            cols,
            rows,
            scrollback,
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        let old_rows = self.rows as usize;
        self.cols = cols;
        self.rows = rows;
        
        if (rows as usize) > old_rows {
            for _ in 0..(rows as usize - old_rows) {
                self.lines.push(Line::new(cols));
            }
        } else if (rows as usize) < old_rows {
            self.lines.truncate(rows as usize);
        }
        
        for line in &mut self.lines {
            line.resize(cols);
        }
    }

    pub fn get_line(&self, y: usize) -> Option<&Line> {
        self.lines.get(y)
    }

    pub fn get_line_mut(&mut self, y: usize) -> Option<&mut Line> {
        self.lines.get_mut(y)
    }

    pub fn scroll_up(&mut self, count: usize) {
        let scrollback_limit = self.scrollback + self.rows as usize;
        
        for _ in 0..count {
            if self.lines.len() >= scrollback_limit {
                self.lines.remove(0);
            }
            self.lines.push(Line::new(self.cols));
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        for _ in 0..self.rows {
            self.lines.push(Line::new(self.cols));
        }
    }

    pub fn clear_line(&mut self, y: usize) {
        if let Some(line) = self.lines.get_mut(y) {
            line.clear();
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if let Some(line) = self.lines.get_mut(y) {
            line.set_cell(x, cell);
        }
    }

    pub fn detect_directions(&mut self) {
        let analyzer = BidiAnalyzer::new();
        for line in &mut self.lines {
            line.direction = analyzer.analyze_line(&line.cells);
        }
    }
}

impl Line {
    pub fn new(cols: u16) -> Self {
        Line {
            cells: vec![Cell::default(); cols as usize],
            direction: Direction::LTR,
        }
    }

    pub fn resize(&mut self, cols: u16) {
        let new_len = cols as usize;
        if new_len > self.cells.len() {
            self.cells.resize(new_len, Cell::default());
        } else {
            self.cells.truncate(new_len);
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    pub fn set_cell(&mut self, x: usize, cell: Cell) {
        if x < self.cells.len() {
            self.cells[x] = cell;
        }
    }
}
