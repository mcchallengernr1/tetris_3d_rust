use macroquad::prelude::*;

use crate::camera::Camera;
use crate::field::{CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z};

pub enum Direction {
    XPlus,
    XMinus,
    YPlus,
    YMinus,
    ZPlus,
    ZMinus
}

pub enum Dir {
    X,
    Y,
    Z
}

pub fn from_i32_to_f32(pos: [i32; 3]) -> Vec3 {
    Vec3::new(pos[0] as f32, pos[1] as f32, pos[2] as f32)
}

pub fn from_f32_to_i32(pos: Vec3) -> [i32; 3] {
    [pos[0].floor() as i32, pos[1].floor() as i32, pos[2].floor() as i32]
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris 3D".to_owned(),
        window_width: 1900,
        window_height: 1000,
        ..Default::default()
    }
}

pub fn in_field(pos: [i32; 3]) -> bool {
    (0 <= pos[0] && CELLS_IN_X > pos[0]) && (0 <= pos[1] && CELLS_IN_Y > pos[1]) && (0 <= pos[2] && CELLS_IN_Z > pos[2])
} 

pub trait Renderable {
    fn draw(&self, cam: &Camera);
    fn dist_to_pos(&self, pos: Vec3) -> f32;
}

pub trait Movable {
    fn move_(&mut self, movement: Vec3);
}
    // pub fn dist_to_pos(&self, pos: Vec3) -> f32 {
    //     (self.mid_pos - pos).length()
    // }