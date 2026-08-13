mod point;
mod camera;
mod utils;
mod face;
mod cube;
mod segment;
mod field;
mod line;
mod piece;

use crate::field::Field;
use crate::camera::Camera;
use crate::utils::window_conf;
use crate::piece::Piece;

use macroquad::math::Vec3;
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

    let mut cam = Camera::new(screen_width(), screen_height());
    cam.update_internal_vars();

    let mut last_mouse_position = mouse_position_local();

    // let mut cubes = vec![cube1, cube2];

    // let p1 = Point::new(Vec3::ZERO);
    // let p2 = Point::new(Vec3::new(0.0, 0.0, 1.0));
    // let segment = Segment::new(p1, p2);

    let piece = Piece::new(0);

    let field = Field::new();

    let _line = line::Line::new(Vec3::ZERO, 3, line::Dir::Z);

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

        cam.draw(&piece, &field);
        
        next_frame().await
    }
}
