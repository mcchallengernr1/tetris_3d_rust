use crate::segment::Segment;
use crate::point::Point;
use crate::utils::Renderable;
use macroquad::math::Vec3;
use crate::C_S;

pub enum Dir {
    X,
    Y,
    Z
}

pub struct Line {
    segments: Vec<Segment>,
    pub mid_pos: Vec3,
}

impl Line {
    pub fn new(pos: Vec3, length: u32, dir: Dir) -> Line {
        let mut segments = Vec::new();
        let mid_pos = match &dir {
            Dir::X => Vec3 { x: pos[0] + (length) as f32 / 2.0, y: pos[1], z: pos[2] },
            Dir::Y => Vec3 { x: pos[0], y: pos[1] + (length) as f32 / 2.0, z: pos[2] },
            Dir::Z => Vec3 { x: pos[0], y: pos[1], z: pos[2] + (length) as f32 / 2.0 },
        };

        for i in 0..length {
            let point_pos = match &dir {
                Dir::X => [Vec3::new(pos[0] + i as f32 * C_S, pos[1], pos[2]), Vec3::new(pos[0] + i as f32 * C_S + C_S, pos[1], pos[2])],
                Dir::Y => [Vec3::new(pos[0], pos[1] + i as f32 * C_S, pos[2]), Vec3::new(pos[0], pos[1] + i as f32 * C_S + C_S, pos[2])],
                Dir::Z => [Vec3::new(pos[0], pos[1], pos[2] + i as f32 * C_S), Vec3::new(pos[0], pos[1], pos[2] + i as f32 * C_S + C_S)]
            };
            segments.push(Segment::new(Point::new(point_pos[0]), Point::new(point_pos[1])));    
        }
        Line { segments, mid_pos }
    }
}

impl Renderable for Line {
    fn draw(&self, cam: &crate::camera::Camera) {
        self.segments.iter().for_each(|s| s.draw(&cam));
    }

    fn dist_to_pos(&self, pos: Vec3) -> f32 {
         (self.mid_pos - pos).length()
    }
}