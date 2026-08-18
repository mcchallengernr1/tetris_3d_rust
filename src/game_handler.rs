use macroquad::math::{Vec2, Vec3};

use macroquad::{input::{MouseButton, is_mouse_button_down, mouse_position_local, mouse_wheel}, prelude::{KeyCode, is_key_pressed}};

use crate::camera::Camera;
use crate::piece::Piece;
use crate::field::Field;

#[derive(PartialEq)]
enum Scroll {
    Up,
    Down,
    Not,
}

#[derive(PartialEq)]
enum Action {
    FrontBack,
    LeftRight,
    FlipFBAxis,
    FlipLRAxis,
    FlipZAxis,
    NoAction,
}

pub struct GameHandler {
    pub running: bool,
    pub mouse_pos: Vec2,
    last_mouse_pos: Vec2,
    pub mouse_displacement: Vec2,
    scroll: Scroll,
    action: Action,
}

impl GameHandler {
    pub fn new() -> GameHandler {
        GameHandler {
            running: true,
            mouse_pos: Vec2::ZERO,
            last_mouse_pos: Vec2::ZERO,
            mouse_displacement: Vec2::ZERO,
            scroll: Scroll::Not,
            action: Action::NoAction,
        }
    }

    pub fn events(&mut self, cam: &mut Camera) {
        self.mouse_pos = mouse_position_local();
        self.mouse_displacement = self.last_mouse_pos - self.mouse_pos;
        self.last_mouse_pos = self.mouse_pos;

        if is_key_pressed(KeyCode::Tab) {self.running = false;}
        if is_mouse_button_down(MouseButton::Left) {
            cam.spherical_movement(self.mouse_displacement);
            cam.update_internal_vars();}

        self.scroll = match mouse_wheel().1 {
            -1.0 => Scroll::Down,
            0.0 => Scroll::Not,
            1.0 => Scroll::Up,
            _ => {println!("Scroll amount of range"); Scroll::Not},
        };

        if is_key_pressed(KeyCode::Y) {self.action = Action::FlipLRAxis}
        else if is_key_pressed(KeyCode::X) {self.action = Action::FlipFBAxis}
        else if is_key_pressed(KeyCode::C) {self.action = Action::FlipZAxis}
        else if is_key_pressed(KeyCode::S) {self.action = Action::LeftRight}
        else if is_key_pressed(KeyCode::D) {self.action = Action::FrontBack}
        else {self.action = Action::NoAction};
    }

    pub fn update_piece(&self, cam: &Camera, field: &Field, piece: &mut Piece){
        if self.scroll == Scroll::Down || self.scroll == Scroll::Up {
            match cam.quadrant {
                0 => {if self.scroll == Scroll::Up {piece.test_move(field, Vec3::new(1.0, 0.0, 0.0));} 
                else if self.scroll == Scroll::Down {piece.test_move(field, Vec3::new(-1.0, 0.0, 0.0));}},
                1 => {if self.scroll == Scroll::Up {piece.test_move(field, Vec3::new(0.0, -1.0, 0.0));}
                else if self.scroll == Scroll::Down {piece.test_move(field, Vec3::new(0.0, 1.0, 0.0));}},
                2 => {if self.scroll == Scroll::Up {piece.test_move(field, Vec3::new(-1.0, 0.0, 0.0));}
                else if self.scroll == Scroll::Down {piece.test_move(field, Vec3::new(1.0, 0.0, 0.0));}},
                3 => {if self.scroll == Scroll::Up {piece.test_move(field, Vec3::new(0.0, 1.0, 0.0));}
                else if self.scroll == Scroll::Down {piece.test_move(field, Vec3::new(0.0, -1.0, 0.0));}},
                _ => println!("Cam Quadrant: {} not handled", cam.quadrant)}
        }
    }
}