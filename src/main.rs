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

use macroquad::{color::PURPLE, prelude::{next_frame, screen_height, screen_width}};

const C_S: f32 = 1.0;
const C_H_S: f32 = C_S / 2.0;

pub const CELLS_IN_X: i32 = 9;
pub const CELLS_IN_Y: i32 = 9;
pub const CELLS_IN_Z: i32 = 20;

#[macroquad::main(window_conf)]
async fn main() {

    let mut game_handler = GameHandler::new(true);
    let mut cam = Camera::new(screen_width(), screen_height());
    cam.update_internal_vars();

    let mut frames: u32 = 0;

    let mut piece = Piece::new(30);

    let mut field = Field::new(PURPLE); 
    // field.fill_field(1);

    while game_handler.running {
        // Events
        game_handler.events(&mut cam);

        // Logic
        game_handler.update_piece(cam.quadrant, &field, &mut piece);

        frames += 1;

        if frames % 10 == 0 && !game_handler.paused {
            if !piece.test_move(&field, [0, 0, -1]) {
                field.add_piece(piece);
                piece = Piece::new(30);
            }
        }


        
        // Display
        cam.clear_screen();

        cam.draw(&piece, &field);

        cam.display_text();
        
        next_frame().await
    }
}
