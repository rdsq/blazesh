#[derive(PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn try_parse(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }
    pub fn to_ansi256(&self) -> u8 {
        fn component(v: u8) -> u8 {
            if v < 48 {
                0
            } else if v < 114 {
                1
            } else {
                ((v - 35) / 40).min(5)
            }
        }
        let r = component(self.r);
        let g = component(self.g);
        let b = component(self.b);
        16 + 36 * r + 6 * g + b
    }
    pub fn to_ansi_foreground(&self) -> String {
        format!("\x1b[38;5;{}m", self.to_ansi256())
    }
}
