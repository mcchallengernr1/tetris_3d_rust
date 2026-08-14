use macroquad::math::Vec2;

use macroquad::{input::{MouseButton, is_mouse_button_down, mouse_position_local, mouse_wheel}, prelude::{KeyCode, is_key_pressed}};

use crate::camera::Camera;
use crate::piece::Piece;


pub struct Game_Handler {
    pub running: bool,
    pub mouse_pos: Vec2,
    last_mouse_pos: Vec2,
    pub mouse_displacement: Vec2,
    scroll: (f32, f32),
}

impl Game_Handler {
    pub fn new() -> Game_Handler {
        Game_Handler {
            running: true,
            mouse_pos: Vec2::ZERO,
            last_mouse_pos: Vec2::ZERO,
            mouse_displacement: Vec2::ZERO,
            scroll: (0.0, 0.0)
        }
    }

    pub fn events(&mut self, cam: &mut Camera,  piece: &mut Piece) {
        self.mouse_pos = mouse_position_local();
        self.mouse_displacement = self.last_mouse_pos - self.mouse_pos;
        self.last_mouse_pos = self.mouse_pos;
        self.scroll = mouse_wheel();

        if is_key_pressed(KeyCode::Tab) {self.running = false;}
        if is_mouse_button_down(MouseButton::Left) {
            cam.spherical_movement([self.last_mouse_pos[0] - self.mouse_pos[0], self.last_mouse_pos[1] - self.mouse_pos[1]]);
            cam.update_internal_vars();}

        if self.scroll != (0.0, 0.0) {println!("Scroll: {0},   {1}", self.scroll.0, self.scroll.1);}
    }
}