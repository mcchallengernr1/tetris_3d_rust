use macroquad::math::{IVec3, Vec3};

use crate::utils::Movable;

#[derive(Copy, Clone)]
pub struct Point {
    pub pos: Vec3,
}

impl Point {
    pub fn new(pos: Vec3) -> Point {
        Point { pos }
    }
}

impl Movable for Point {
    fn move_(&mut self, mov: IVec3) {
        self.pos += mov.as_vec3();
    }
}