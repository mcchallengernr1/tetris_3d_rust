mod point;
mod camera;
mod utils;
mod face;
mod cube;
mod segment;
mod field;
mod line;
mod piece;
mod game_handler;

use std::println;

use crate::cube::Cube;
use crate::field::Field;
use crate::camera::Camera;
use crate::game_handler::Game_Handler;
use crate::utils::window_conf;
use crate::piece::Piece;

use macroquad::color::RED;
use macroquad::{input::{MouseButton, is_mouse_button_down, mouse_position_local}, prelude::{
    KeyCode, is_key_pressed, next_frame, screen_height, screen_width
}};

const C_S: f32 = 1.0;
const C_H_S: f32 = C_S / 2.0;

#[macroquad::main(window_conf)]
async fn main() {
    
    // let cube1 = Cube::new([0, 0, 0], RED);
    // let cube2 = Cube::new([0, 0, 2], BLUE);
    // let face_array = [Face::new([0, 0, 0], XMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], XPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }), Face::new([0, 0, 0], YMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], YPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }), Face::new([0, 0, 0], ZMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], ZPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 })];

    let mut game_handler = Game_Handler::new();
    let mut cam = Camera::new(screen_width(), screen_height());
    cam.update_internal_vars();


    let piece = Piece::new(0);

    let mut field = Field::new(); 

    field.cubes.push(Cube::new([1, 1, 10], RED));

    loop {
        let mouse_pos = mouse_position_local();
        if is_key_pressed(KeyCode::Tab) {
            break
        } if is_key_pressed(KeyCode::Right) {
            piece = Piece::new(piece.n + 1);
            println!("{}", piece.n);
        } if is_key_pressed(KeyCode::Left) {
            piece = Piece::new(piece.n - 1);
            println!("{}", piece.n);
        } if is_mouse_button_down(MouseButton::Left) {
        cam.spherical_movement([last_mouse_position[0] - mouse_pos[0], last_mouse_position[1] - mouse_pos[1]]);
        cam.update_internal_vars();
        }
        last_mouse_position = mouse_pos;


        // Display
        cam.clear_screen();

        cam.draw(&piece, &field);
        
        next_frame().await
    }
}
