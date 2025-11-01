use rgb::RGB8;

fn mix(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

pub fn gradient(colors: &Vec<RGB8>, position: f32) -> RGB8 {
    let pos1 = position.floor() as usize % colors.len();
    let pos2 = (pos1 + 1) % colors.len();
    let c1 = &colors[pos1];
    let c2 = &colors[pos2];
    let progress = position % 1.0;
    RGB8 {
        r: mix(c1.r, c2.r, progress),
        g: mix(c1.g, c2.g, progress),
        b: mix(c1.b, c2.b, progress),
    }
}
