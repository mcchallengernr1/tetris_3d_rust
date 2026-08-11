use macroquad::prelude::*;

use crate::camera::Camera;

pub fn from_i32_to_f32(pos: [i32; 3]) -> Vec3 {
    Vec3::new(pos[0] as f32, pos[1] as f32, pos[2] as f32)
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris 3D".to_owned(),
        window_width: 1900,
        window_height: 1000,
        ..Default::default()
    }
}

pub trait Renderable {
    fn draw(&self, cam: &Camera);
}