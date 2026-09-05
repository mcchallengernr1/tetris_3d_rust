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

use crate::field::Field;
use crate::camera::Camera;
use crate::game_handler::GameHandler;
use crate::utils::window_conf;
use crate::piece::Piece;

use macroquad::{color::PURPLE, prelude::next_frame, rand, miniquad::date::now, math::IVec3};

const C_S: f32 = 1.0;
const C_H_S: f32 = C_S / 2.0;

pub const CELLS_IN_X: usize = 9;
pub const CELLS_IN_Y: usize = 9;
pub const CELLS_IN_Z: usize = 20;

const CAMERA_RADIUS: f32 = 50.0;
const AXIS_LENGTH: usize = 9;

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(now() as u64);
    
    let mut game_handler = GameHandler::new(true);
    let mut cam = Camera::new();
    cam.update_internal_vars();

    let mut frames: u32 = 0;

    let mut piece = Piece::new_random();

    let mut field = Field::new(PURPLE); 
    // field._fill_field_to_percent(50);

    while game_handler.running {
        // Events
        game_handler.events(&mut cam);

        // Logic
        game_handler.update_piece(cam.quadrant, &field, &mut piece);

        frames += 1;

        if frames.is_multiple_of(100) && !game_handler.paused && !piece.try_move(&field, IVec3::ZERO.with_z(-1)) {
            field.add_piece(piece);
            piece = Piece::new_random();
        }


        // Display
        cam.update_internal_vars();

        cam.clear_screen();

        cam.draw(&piece, &field);

        cam.display_text(&piece);
        
        next_frame().await
    }
}
