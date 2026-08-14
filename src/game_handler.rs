use macroquad::math::Vec2;

use macroquad::{input::{MouseButton, is_mouse_button_down, mouse_position_local}, prelude::{
    KeyCode, is_key_pressed, next_frame, screen_height, screen_width
}};

use crate::camera::Camera;


pub struct Game_Handler {
    pub mouse_pos: Vec2,
    last_mouse_pos: Vec2,
    pub mouse_displacement: Vec2,
}

impl Game_Handler {
    pub fn new() -> Game_Handler {
        Game_Handler {
            mouse_pos: Vec2::ZERO,
            last_mouse_pos: Vec2::ZERO,
            mouse_displacement: Vec2::ZERO,
        }
    }

    pub fn events(&mut self, cam: &mut Camera) {
        self.mouse_pos = mouse_position_local();
        self.mouse_displacement = self.last_mouse_pos - self.mouse_pos;
        self.last_mouse_pos = self.mouse_pos;

        if is_key_pressed(KeyCode::Tab) {
            
        } if is_mouse_button_down(MouseButton::Left) {
        cam.spherical_movement([self.last_mouse_pos[0] - self.mouse_pos[0], self.last_mouse_pos[1] - self.mouse_pos[1]]);
        cam.update_internal_vars();
        }


    }
}