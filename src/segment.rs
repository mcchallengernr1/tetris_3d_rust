use crate::{Camera, point::Point, utils::{Movable, Renderable}};
use macroquad::{color::Color, math::IVec3, shapes::draw_line};

pub struct Segment {
    points: [Point; 2],
    pub pos: IVec3,
    pub color: Color,
}

impl Segment {
    pub fn new(p1: Point, p2: Point, pos: IVec3, color: Color) -> Segment {
        Segment { points: [p1, p2], 
            pos,
            color }
    }

}

impl Renderable for Segment {
    fn draw(&self, cam: &Camera) {
        let p0 = cam.project(self.points[0].pos);
        let p1 = cam.project(self.points[1].pos);
        draw_line(p0[0], p0[1], p1[0], p1[1], 1.0, self.color);
    }
}

impl Movable for Segment {
    fn move_(&mut self, mov: IVec3) {
        self.points[0].move_(mov);
        self.points[1].move_(mov);
        self.pos += mov;
    }
}