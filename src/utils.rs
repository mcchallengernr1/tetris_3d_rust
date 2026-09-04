use macroquad::prelude::{Vec3, Conf, miniquad::conf::Platform};

use crate::camera::Camera;
use crate::{CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z};
use FaceNormal::*;

pub enum FaceNormal {
    XPlus,
    XMinus,
    YPlus,
    YMinus,
    ZPlus,
    ZMinus
}

impl FaceNormal {
    pub fn to_index(&self) -> usize {
        match self {
            XMinus => 0,
            XPlus => 1,
            YMinus => 2,
            YPlus => 3,
            ZMinus => 4,
            ZPlus => 5
        }
    }
}

pub enum Axis {
    X,
    Y,
    Z
}

pub fn from_i32_to_f32(pos: [i32; 3]) -> Vec3 {
    Vec3::new(pos[0] as f32, pos[1] as f32, pos[2] as f32)
}

pub fn _from_f32_to_i32(pos: Vec3) -> [i32; 3] {
    [pos[0].round() as i32, pos[1].round() as i32, pos[2].round() as i32]
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris 3D".to_owned(),
        window_width: 1900,
        window_height: 1000,
        platform: Platform {
          swap_interval: Some(0),
          ..Default::default()
        },
        ..Default::default()
    }
}

pub fn in_field(pos: [i32; 3]) -> bool {
    (0 <= pos[0] && CELLS_IN_X as i32 > pos[0]) && (0 <= pos[1] && CELLS_IN_Y as i32 > pos[1]) && (0 <= pos[2] && CELLS_IN_Z as i32 > pos[2])
} 

pub fn add_i32_vec(v1: [i32; 3], v2: [i32; 3]) -> [i32; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

pub fn sub_i32_vec(v1: [i32; 3], v2: [i32; 3]) -> [i32; 3] {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}

pub fn is_x_looking(quadrant: i32) -> bool {
    quadrant == 7 || quadrant == 0 || quadrant == 3 || quadrant == 4
}

pub trait Renderable {
    fn draw(&self, cam: &Camera);
    fn dist_to_pos(&self, pos: Vec3) -> f32;
}

pub trait Movable {
    fn move_(&mut self, movement: Vec3);
}