use macroquad::color::Color;
use macroquad::math::IVec3;
use macroquad::prelude::{Vec3, Conf, miniquad::conf::Platform};
use macroquad::rand::gen_range;

use crate::camera::Camera;
use crate::{CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z};
use FaceNormal::*;

#[derive(PartialEq)]
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

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris 3D".to_owned(),
        window_width: crate::INITIAL_WINDOW_WIDTH,
        window_height: crate::INITIAL_WINDOW_HEIGHT,
        platform: Platform {
          swap_interval: if crate::UNLOCKED_FPS {Some(0)} else {None},
          ..Default::default()
        },
        ..Default::default()
    }
}

pub fn in_field(pos: IVec3) -> bool {
    (0 <= pos[0] && CELLS_IN_X as i32 > pos[0]) && (0 <= pos[1] && CELLS_IN_Y as i32 > pos[1]) && (0 <= pos[2] && CELLS_IN_Z as i32 > pos[2])
} 

pub fn is_x_looking(quadrant: i32) -> bool {
    quadrant == 7 || quadrant == 0 || quadrant == 3 || quadrant == 4
}

pub fn _random_color() -> Color {
    Color::from_rgba(gen_range(0, 255), gen_range(0, 255), gen_range(0, 255), 255)
}

pub trait Renderable {
    fn draw(&self, cam: &Camera);
    fn dist_to_pos(&self, pos: Vec3) -> f32;
}

pub trait Movable {
    fn move_(&mut self, movement: Vec3);
}