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

use macroquad::{color::PURPLE, math::IVec3, miniquad::date::now, prelude::next_frame, rand};

const C_S: f32 = 1.0;
const C_H_S: f32 = C_S / 2.0;

const CELLS_IN_X: usize = 7;
const CELLS_IN_Y: usize = 7;
const CELLS_IN_Z: usize = 20;

const CAMERA_RADIUS: f32 = 50.0;
const AXIS_LENGTH: usize = 9;

const UNLOCKED_FPS: bool = false;
const INITIAL_WINDOW_WIDTH: i32 = 2100;
const INITIAL_WINDOW_HEIGHT: i32 = 1470;

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(now() as u64);
    
    let mut game_handler = GameHandler::new(false);
    let mut cam = Camera::new();
    cam.update_internal_vars();

    let mut piece = Piece::new(0);

    let mut field = Field::new(PURPLE);
    // field._fill_field_to_percent(50, 8);

    while game_handler.running {
        // Events
        game_handler.events(&mut cam);

        // Logic
        game_handler.regulate_speed();

        game_handler.update_piece(cam.quadrant, &field, &mut piece);

        if game_handler.sinks && !piece.try_move(&field, IVec3::ZERO.with_z(-1)) {
            field.add_piece(piece);
            let _number_of_cleared_lines = field.try_line_clear();
            piece = Piece::new_random();

        }


        // Display
        cam.update_internal_vars();

        cam.clear_screen();

        cam.draw(&piece, &field);

        cam.display_text(&piece, game_handler.get_fps());
        
        next_frame().await
    }
}
