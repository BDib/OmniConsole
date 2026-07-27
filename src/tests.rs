#[cfg(test)]
mod tests {
    use crate::bidi::detector::is_rtl_char;
    use crate::bidi::reorder::get_display_cells;
    use crate::terminal::{Cell, Direction};

    #[test]
    fn test_is_rtl_char() {
        // Arabic letter Alif
        assert!(is_rtl_char('ا'));
        // Hebrew letter Alef
        assert!(is_rtl_char('א'));
        // English letter A
        assert!(!is_rtl_char('A'));
    }

    #[test]
    fn test_arabic_reshaper() {
        let input = "سلام";
        let reshaped = arabic_reshaper::arabic_reshape(input);
        // The reshaped form should contain properly shaped Arabic letters
        assert_ne!(reshaped, input);
        // "سلام" contains Lam + Alif which gets combined into a single Lam-Alif ligature glyph, so length goes from 4 to 3!
        assert_eq!(reshaped.chars().count(), 3);
    }

    #[test]
    fn test_bidi_visual_reordering() {
        let cells = vec![
            Cell { ch: 'ا', ..Default::default() },
            Cell { ch: 'ل', ..Default::default() },
            Cell { ch: 'ع', ..Default::default() },
            Cell { ch: 'ر', ..Default::default() },
        ];

        // Perform display cell reordering with LTR (logical) and RTL (reversed) directions
        let ltr_display = get_display_cells(&cells, Direction::LTR);
        assert_eq!(ltr_display.len(), 4);
        assert_eq!(ltr_display[0].cell.ch, 'ﺍ'); // Reshaped Alif isolated/initial

        let rtl_display = get_display_cells(&cells, Direction::RTL);
        assert_eq!(rtl_display.len(), 4);
    }
}
