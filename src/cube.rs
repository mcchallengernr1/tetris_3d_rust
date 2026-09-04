use crate::face::Face;
use crate::camera::Camera;
use crate::C_H_S;
use crate::utils::{FaceNormal::*, Movable, Renderable, add_i32_vec, from_i32_to_f32};
use macroquad::color::Color;
use macroquad::math::Vec3;


pub struct Cube {
    pub faces: [Face; 6],
    pub pos: [i32; 3],
    mid_pos: Vec3,
}

impl Cube {
    pub fn new(pos: [i32; 3], color: Color) -> Cube {
        let faces = [
            Face::new(pos, XMinus, color),
            Face::new(pos, XPlus, color),
            Face::new(pos, YMinus, color),
            Face::new(pos, YPlus, color),
            Face::new(pos, ZMinus, color),
            Face::new(pos, ZPlus, color)];

        let mid_pos = Vec3::new(pos[0] as f32 + C_H_S,pos[1] as f32 + C_H_S, pos[2] as f32 + C_H_S);

        Cube { faces , pos , mid_pos }
    }
    pub fn goto(&mut self, pos: [i32; 3]) {
        self.move_([pos[0] - self.pos[0], pos[1] - self.pos[1], pos[2] - self.pos[2]])
    }

    pub fn move_(&mut self, mov: [i32; 3]) {
        let movement = from_i32_to_f32(mov);
        self.mid_pos += movement;
        self.faces.iter_mut().for_each(|f| f.move_(movement));
        self.pos = add_i32_vec(self.pos, mov)
    }
}

impl Renderable for Cube {
    fn draw(&self, cam: &Camera) {
        self.faces.iter().for_each(|f| f.draw(cam));
    }

    fn dist_to_pos(&self, pos: Vec3) -> f32 {
        (self.mid_pos - pos).length()
    }
}