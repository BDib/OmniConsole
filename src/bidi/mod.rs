pub mod reorder;
pub mod detector;

use super::terminal::{Cell, Direction};

pub struct BidiAnalyzer {
    rtl_threshold: f32,
}

impl BidiAnalyzer {
    pub fn new() -> Self {
        BidiAnalyzer {
            rtl_threshold: 0.3,
        }
    }

    pub fn analyze_line(&self, cells: &[Cell]) -> Direction {
        let text: String = cells.iter().map(|c| c.ch).collect();
        
        if text.trim().is_empty() {
            return Direction::LTR;
        }

        if detector::contains_rtl_markers(&text) {
            return Direction::RTL;
        }

        let rtl_ratio = detector::calculate_rtl_ratio(&text);
        
        if rtl_ratio >= self.rtl_threshold {
            Direction::RTL
        } else {
            Direction::LTR
        }
    }
}

impl Default for BidiAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
