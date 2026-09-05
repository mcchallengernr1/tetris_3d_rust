use crate::{Camera, point::Point, utils::{Movable, Renderable}};
use macroquad::{color::Color, math::Vec3, shapes::draw_line};

pub struct Segment {
    points: [Point; 2],
    mid_pos: Vec3,
    pub color: Color,
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
        let p0 = cam.project(self.points[0].pos);
        let p1 = cam.project(self.points[1].pos);
        draw_line(p0[0], p0[1], p1[0], p1[1], 1.0, self.color);
    }
    
    fn dist_to_pos(&self, pos: Vec3) -> f32 {
        (self.mid_pos - pos).length()
    }
}

impl Movable for Segment {
    fn move_(&mut self, movement: Vec3) {
        self.points[0].move_(movement);
        self.points[1].move_(movement);
        self.mid_pos += movement;
    }
}