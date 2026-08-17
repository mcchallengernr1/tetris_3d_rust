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

    let mut game_handler = Game_Handler::new();
    let mut cam = Camera::new(screen_width(), screen_height());
    cam.update_internal_vars();


    let mut piece = Piece::new(0);

    let mut field = Field::new(); 

    field.cubes.push(Cube::new([1, 1, 10], RED));

    while game_handler.running {
        
        game_handler.events(&mut cam, &mut piece);

        // Display
        cam.clear_screen();

        cam.draw(&piece, &field);
        
        next_frame().await
    }
}
