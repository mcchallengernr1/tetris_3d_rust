use crate::{Point, utils::Renderable, Camera};
use macroquad::{math::Vec3, color::Color, shapes::draw_line};

pub struct Segment {
    points: [Point; 2],
    mid_pos: Vec3,
    color: Color,
}

impl Segment {
    pub fn new(p1: Point, p2: Point, color: Color) -> Segment {
        let mid_pos = (p1.pos + p2.pos) / 2.0;

        Segment { points: [p1, p2], 
            mid_pos,
            color }
    }
}

impl Renderable for Segment {
    fn draw(&self, cam: &Camera) {
        let p0 = cam.project(&self.points[0]);
        let p1 = cam.project(&self.points[1]);
        draw_line(p0[0], p0[1], p1[0], p1[1], 1.0, self.color);
    }
}