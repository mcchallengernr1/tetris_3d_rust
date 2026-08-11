mod point;
mod camera;
mod utils;
mod face;
mod cube;


use std::println;

use crate::camera::Camera;
use crate::cube::Cube;
use crate::utils::{window_conf, Renderable};

use macroquad::{color::Color, input::{MouseButton, is_mouse_button_down, mouse_position_local}, prelude::{
    KeyCode, is_key_pressed, next_frame, screen_height, screen_width
}};

const C_S: f32 = 1.0;
const C_H_S: f32 = C_S / 2.0;

#[macroquad::main(window_conf)]
async fn main() {
    
    let cube1 = Cube::new([0, 0, 0], Color { r: 0.8, g: 0.3, b: 0.5, a: 1.0 });
    let cube2 = Cube::new([0, 0, 2], Color { r: 0.1, g: 0.3, b: 0.5, a: 1.0 });
    // let face_array = [Face::new([0, 0, 0], XMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], XPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }), Face::new([0, 0, 0], YMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], YPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }), Face::new([0, 0, 0], ZMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], ZPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 })];

    let mut cam = Camera::new(screen_width(), screen_height());
    cam.update_internal_vars();

    let mut last_mouse_position = mouse_position_local();

    let mut cubes = vec![cube1, cube2];

    loop {
        let mouse_pos = mouse_position_local();
        if is_key_pressed(KeyCode::Tab) {
            break
        } if is_mouse_button_down(MouseButton::Left) {
        cam.spherical_movement([last_mouse_position[0] - mouse_pos[0], last_mouse_position[1] - mouse_pos[1]]);
        cam.update_internal_vars();
        }
        last_mouse_position = mouse_pos;


        // Display
        cam.clear_screen();

        // println!("{}", cube1.dist_to_pos(cam.pos));
        for cube in &cubes {
            cube.draw(&cam);
        }
        // cube1.draw(&cam);
        // cube2.draw(&cam);

        next_frame().await
    }
}
