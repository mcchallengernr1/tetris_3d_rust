use macroquad::color::{GREEN, RED};
use macroquad::math::Vec2;

use macroquad::{input::{MouseButton, is_mouse_button_down, mouse_position_local, mouse_wheel}, prelude::{KeyCode, is_key_pressed, is_key_down}};

use crate::camera::Camera;
use crate::game_handler::Action::*;
use crate::utils::{Axis, is_x_looking};
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
    None,
}

pub struct GameHandler {
    pub running: bool,
    mouse_pos: Vec2,
    last_mouse_pos: Vec2,
    mouse_displacement: Vec2,
    scroll: Scroll,
    action: Action,
    pub paused: bool,

}

impl GameHandler {
    pub fn new(paused: bool) -> GameHandler {
        GameHandler {
            running: true,
            mouse_pos: Vec2::ZERO,
            last_mouse_pos: Vec2::ZERO,
            mouse_displacement: Vec2::ZERO,
            scroll: Scroll::Not,
            action: Action::None,
            paused
        }
    }

    pub fn events(&mut self, cam: &mut Camera) {
        self.mouse_pos = mouse_position_local();
        self.mouse_displacement = self.last_mouse_pos - self.mouse_pos;
        self.last_mouse_pos = self.mouse_pos;

        if is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::Escape) {self.running = false;}
        if is_key_pressed(KeyCode::Space) {self.paused = !self.paused;}
        if is_mouse_button_down(MouseButton::Left) {cam.spherical_movement(self.mouse_displacement);}

        self.scroll = match mouse_wheel().1 {
            -1.0 => Scroll::Down,
            0.0 => Scroll::Not,
            1.0 => Scroll::Up,
            _ => {println!("Scroll amount of range"); Scroll::Not},
        };

        if is_key_down(KeyCode::Period) {self.action = Action::FlipLRAxis}
        else if is_key_down(KeyCode::X) {self.action = Action::FlipFBAxis}
        else if is_key_down(KeyCode::Y) {self.action = Action::FlipZAxis}
        else if is_key_down(KeyCode::I) {self.action = Action::LeftRight}
        else if is_key_down(KeyCode::E) {self.action = Action::FrontBack}
        else {self.action = Action::None};
    }

    pub fn update_piece(&self, quadrant: i32, field: &Field, piece: &mut Piece){
        piece.turn_off_axies();
        piece.turn_on_axies(match self.action {
            FlipFBAxis => if is_x_looking(quadrant) {1} else {0},
            FlipLRAxis => if is_x_looking(quadrant) {0} else {1},
            LeftRight => if is_x_looking(quadrant) {1} else {0},
            FrontBack => if is_x_looking(quadrant) {0} else {1},
            FlipZAxis => 2,
            None => 4,
        });

        if self.action != None && self.scroll != Scroll::Not {
            let forwards = match self.scroll {Scroll::Down => false, Scroll::Up => true, Scroll::Not => false};

            if self.action == FlipZAxis {
                piece.try_rotate(field, Axis::Z, forwards);
            } else {
                if quadrant == 7 || quadrant == 0 {
                    if self.action == LeftRight {piece.try_move(field, [0, if forwards {1} else {-1}, 0]);}
                    else if self.action == FrontBack {piece.try_move(field, [if forwards {-1} else {1}, 0, 0]);}
                    else if self.action == FlipFBAxis {piece.try_rotate(field, Axis::Y, !forwards);}
                    else if self.action == FlipLRAxis {piece.try_rotate(field, Axis::X, forwards);}
                
                } else if quadrant == 1 || quadrant == 2 {
                    if self.action == LeftRight {piece.try_move(field, [if forwards {-1} else {1}, 0, 0]);}
                    else if self.action == FrontBack {piece.try_move(field, [0, if forwards {-1} else {1}, 0]);}
                    else if self.action == FlipFBAxis {piece.try_rotate(field, Axis::X, forwards);}
                    else if self.action == FlipLRAxis {piece.try_rotate(field, Axis::Y, forwards);}

                } else if quadrant == 3 || quadrant == 4 {
                    if self.action == LeftRight {piece.try_move(field, [0, if forwards {-1} else {1}, 0]);}
                    else if self.action == FrontBack {piece.try_move(field, [if forwards {1} else {-1}, 0, 0]);}
                    else if self.action == FlipFBAxis {piece.try_rotate(field, Axis::Y, forwards);}
                    else if self.action == FlipLRAxis {piece.try_rotate(field, Axis::X, !forwards);}

                } else {
                    if self.action == LeftRight {piece.try_move(field, [if forwards {1} else {-1}, 0, 0]);}
                    else if self.action == FrontBack {piece.try_move(field, [0, if forwards {1} else {-1}, 0]);}
                    else if self.action == FlipFBAxis {piece.try_rotate(field, Axis::X, !forwards);}
                    else if self.action == FlipLRAxis {piece.try_rotate(field, Axis::Y, !forwards);}
                }
            }            
        }
        if is_x_looking(quadrant) {
            piece.axies[0].change_color(RED);
            piece.axies[1].change_color(GREEN);
        } else {
            piece.axies[0].change_color(GREEN);
            piece.axies[1].change_color(RED);
        }
    }
}