use macroquad::math::Vec3;

use crate::utils::Movable;

pub struct Point {
    pub pos: Vec3,
}

impl Point {
    pub fn new(pos: Vec3) -> Point {
        Point { pos }
    }
}

impl Movable for Point {
    fn move_(&mut self, movement: Vec3) {
        self.pos += movement;
    }
}