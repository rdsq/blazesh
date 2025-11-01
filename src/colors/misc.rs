use ansi_colours::ansi256_from_rgb;
use rgb::RGB8;

pub fn try_parse_hex(hex: &str) -> Option<RGB8> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(RGB8 { r, g, b })
}

pub fn to_ansi_foreground(rgb: &RGB8) -> String {
    format!("\x1b[38;5;{}m", ansi256_from_rgb(rgb))
}
