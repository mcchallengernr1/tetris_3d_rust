use crate::{Point, utils::Renderable, Camera};
use macroquad::{color::{Color, PURPLE}, math::Vec3, shapes::draw_line};

pub struct Segment<'a> {
    points: [&'a Point; 2],
    mid_pos: Vec3,
    color: Color,
}

impl<'a> Segment<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point) -> Segment<'a> {
        let mid_pos = (p1.pos + p2.pos) / 2.0;
        let color = PURPLE;
        Segment { points: [p1, p2], 
            mid_pos,
            color }
    }
}

impl<'a> Renderable for Segment<'a> {
    fn draw(&self, cam: &Camera) {
        let p0 = cam.project(&self.points[0]);
        let p1 = cam.project(&self.points[1]);
        draw_line(p0[0], p0[1], p1[0], p1[1], 1.0, self.color);
    }
    
    fn dist_to_pos(&self, pos: Vec3) -> f32 {
        (self.mid_pos - pos).length()
    }
}