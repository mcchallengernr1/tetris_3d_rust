use crate::face::{Face, FaceDirection::*};
use crate::camera::Camera;
use crate::C_H_S;
use crate::utils::Renderable;
use macroquad::color::Color;
use macroquad::math::Vec3;

pub struct Cube {
    faces: [Face; 6],
    pos: [i32; 3],
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
            Face::new(pos, ZPlus, color),];

        let mid_pos = Vec3::new(pos[0] as f32 + C_H_S,pos[1] as f32 + C_H_S, pos[2] as f32 + C_H_S);

        Cube { faces , pos , mid_pos }
    }

    // pub fn draw(&self, cam: &Camera) {
    //     for face in &self.faces {
    //         face.draw(&self.color, cam)
    //     }
    // }

    // pub fn dist_to_pos(&self, pos: Vec3) -> f32 {
    //     (self.mid_pos - pos).length()
    // }

}

impl Renderable for Cube {
    fn draw(&self, cam: &Camera) {
        self.faces.iter().for_each(|f| f.draw(cam));
    }

    fn dist_to_pos(&self, pos: Vec3) -> f32 {
        (self.mid_pos - pos).length()
    }
}