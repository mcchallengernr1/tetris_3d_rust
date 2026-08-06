pub struct Point {
    pub pos: [f32; 3],
    pub color: macroquad::color::Color
}

impl Point {
    pub fn new(pos: [f32; 3], color: macroquad::color::Color) -> Point {
        Point {pos, color}
    }
}