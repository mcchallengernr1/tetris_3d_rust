use macroquad::math::Vec3;

pub struct Point {
    pub pos: Vec3,
}

impl Point {
    pub fn new(pos: Vec3) -> Point {
        Point {pos}
    }
}