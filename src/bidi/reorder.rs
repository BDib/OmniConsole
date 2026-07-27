use unicode_bidi::BidiInfo;
use super::super::terminal::{Cell, Direction};

pub fn get_display_cells(cells: &[Cell], direction: Direction) -> Vec<DisplayCellInfo> {
    let reordered = reorder_for_display(cells, direction);
    
    reordered.into_iter().enumerate().map(|(visual_pos, (logical_pos, cell))| {
        DisplayCellInfo {
            visual_position: visual_pos,
            logical_position: logical_pos,
            cell,
            direction,
        }
    }).collect()
}

fn reorder_for_display(cells: &[Cell], direction: Direction) -> Vec<(usize, Cell)> {
    if cells.is_empty() {
        return vec![];
    }

    let text: String = cells.iter().map(|c| c.ch).collect();
    
    match direction {
        Direction::LTR => {
            cells.iter().enumerate().map(|(i, c)| (i, *c)).collect()
        }
        Direction::RTL => {
            reorder_rtl(cells, &text)
        }
        Direction::Auto => {
            apply_bidi_algorithm(cells, &text)
        }
    }
}

fn reorder_rtl(cells: &[Cell], text: &str) -> Vec<(usize, Cell)> {
    if text.trim().is_empty() {
        return cells.iter().enumerate().map(|(i, c)| (i, *c)).collect();
    }

    let bidi_info = BidiInfo::new(text, None);
    
    if let Some(para) = bidi_info.paragraphs.first() {
        let levels = bidi_info.reordered_levels(para, 0..text.len());
        let visual_order = BidiInfo::reorder_visual(&levels);
        
        let mut result: Vec<(usize, Cell)> = Vec::with_capacity(cells.len());
        for &logical_idx in &visual_order {
            if let Some(cell) = cells.get(logical_idx) {
                result.push((logical_idx, *cell));
            }
        }
        return result;
    }

    let mut result: Vec<(usize, Cell)> = cells.iter().enumerate().map(|(i, c)| (i, *c)).collect();
    result.reverse();
    result
}

fn apply_bidi_algorithm(cells: &[Cell], text: &str) -> Vec<(usize, Cell)> {
    if text.trim().is_empty() {
        return cells.iter().enumerate().map(|(i, c)| (i, *c)).collect();
    }

    let bidi_info = BidiInfo::new(text, None);
    
    if let Some(para) = bidi_info.paragraphs.first() {
        let levels = bidi_info.reordered_levels(para, 0..text.len());
        let visual_order = BidiInfo::reorder_visual(&levels);
        
        let original: Vec<usize> = (0..cells.len()).collect();
        if visual_order == original {
            return cells.iter().enumerate().map(|(i, c)| (i, *c)).collect();
        }
        
        let mut result: Vec<(usize, Cell)> = Vec::with_capacity(cells.len());
        for &logical_idx in &visual_order {
            if let Some(cell) = cells.get(logical_idx) {
                result.push((logical_idx, *cell));
            }
        }
        return result;
    }

    cells.iter().enumerate().map(|(i, c)| (i, *c)).collect()
}

#[derive(Debug, Clone)]
pub struct DisplayCellInfo {
    pub visual_position: usize,
    pub logical_position: usize,
    pub cell: Cell,
    pub direction: Direction,
}
