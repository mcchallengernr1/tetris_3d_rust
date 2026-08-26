use macroquad::math::{Vec2, Vec3};

use macroquad::{input::{MouseButton, is_mouse_button_down, mouse_position_local, mouse_wheel}, prelude::{KeyCode, is_key_pressed, is_key_down}};

use crate::camera::Camera;
use crate::game_handler::Action::*;
use crate::utils::{Direction, Direction::*, Dir};
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

        if is_key_down(KeyCode::Y) {self.action = Action::FlipLRAxis}
        else if is_key_down(KeyCode::X) {self.action = Action::FlipFBAxis}
        else if is_key_down(KeyCode::C) {self.action = Action::FlipZAxis}
        else if is_key_down(KeyCode::S) {self.action = Action::LeftRight}
        else if is_key_down(KeyCode::D) {self.action = Action::FrontBack}
        else {self.action = Action::NoAction};
    }

    pub fn update_piece(&self, _quadrant: i32, field: &Field, piece: &mut Piece){
        if self.action != NoAction && self.scroll != Scroll::Not {
            let forwards = match self.scroll {Scroll::Down => false, Scroll::Up => true, Scroll::Not => false};

            if self.action == FlipZAxis {
                piece.rotate(Dir::Z, forwards);
            } if self.action == FrontBack {
                piece.test_move(field, Vec3::new(if self.scroll == Scroll::Up {1.0} else {-1.0}, 0.0, 0.0));
                
            } 
        }
    }
}