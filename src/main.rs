mod point;
mod camera;
mod utils;
mod face;

use crate::face::FaceDirection::*;
use crate::camera::Camera;
use crate::point::Point;
use crate::face::Face;

use macroquad::{color::Color, input::{MouseButton, is_mouse_button_down, mouse_position_local}, prelude::{
    BLACK, KeyCode, RED, clear_background, draw_circle, is_key_pressed, next_frame, screen_height, screen_width
}};

#[macroquad::main("Tetris 3D")]
async fn main() {

     
    // let point_array = [Point::new(0.0, 0.0, 0.0, RED), Point::new(1.0, 0.0, 0.0, GREEN), point::Point::new(0.0, 1.0, 0.0, BLUE), point::Point::new(1.0, 1.0, 0.0, YELLOW), point::Point::new(0.0, 0.0, 1.0, RED), point::Point::new(1.0, 0.0, 1.0, GREEN), point::Point::new(0.0, 1.0, 1.0, BLUE), point::Point::new(1.0, 1.0, 1.0, YELLOW)];
    let point_array = [Point::new([0.0, 0.0, 0.0], RED)];
    // let point_array = [];
    
    
    let face_array = [Face::new([0, 0, 0], XMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], XPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }), Face::new([0, 0, 0], YMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], YPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }), Face::new([0, 0, 0], ZMinus, Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }), Face::new([0, 0, 0], ZPlus, Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 })];

    let mut cam = Camera::new(screen_width(), screen_height());
    cam.update_internal_vars();

    let mut last_mouse_position = mouse_position_local();

    loop {
        let mouse_pos = mouse_position_local();
        if is_key_pressed(KeyCode::Tab) {
            break
        } if is_mouse_button_down(MouseButton::Middle) {
        cam.spherical_movement([last_mouse_position[0] - mouse_pos[0], last_mouse_position[1] - mouse_pos[1]]);
        cam.update_internal_vars();
        }
        last_mouse_position = mouse_pos;

        clear_background(BLACK);

        

        for face in &face_array {
            face.draw(&cam);
        }


        for point in &point_array {
            let pos = cam.project(&point);
            draw_circle(pos[0], pos[1], 5.0, point.color)
            // draw_circle(x, y, 5.0, point.color)
        }

        next_frame().await
    }
}
