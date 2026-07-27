pub fn contains_rtl_markers(text: &str) -> bool {
    for ch in text.chars() {
        if is_rtl_char(ch) {
            return true;
        }
    }
    false
}

pub fn calculate_rtl_ratio(text: &str) -> f32 {
    let mut rtl_count = 0;
    let mut total_count = 0;

    for ch in text.chars() {
        if ch.is_alphabetic() {
            total_count += 1;
            if is_rtl_char(ch) {
                rtl_count += 1;
            }
        }
    }

    if total_count == 0 {
        0.0
    } else {
        rtl_count as f32 / total_count as f32
    }
}

pub fn is_rtl_char(ch: char) -> bool {
    let code = ch as u32;
    
    // Arabic characters
    if (0x0600..=0x06FF).contains(&code) ||
       (0x0750..=0x077F).contains(&code) ||
       (0x08A0..=0x08FF).contains(&code) ||
       (0xFB50..=0xFDFF).contains(&code) ||
       (0xFE70..=0xFEFF).contains(&code) {
        return true;
    }
    
    // Hebrew characters
    if (0x0590..=0x05FF).contains(&code) ||
       (0xFB1D..=0xFB4F).contains(&code) {
        return true;
    }
    
    // Syriac, Thaana, NKo, Samaritan, Mandaic
    if (0x0700..=0x074F).contains(&code) ||
       (0x0780..=0x07BF).contains(&code) ||
       (0x07C0..=0x07FF).contains(&code) ||
       (0x0800..=0x083F).contains(&code) ||
       (0x0840..=0x085F).contains(&code) {
        return true;
    }
    
    false
}
