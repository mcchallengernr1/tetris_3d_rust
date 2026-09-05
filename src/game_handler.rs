use macroquad::color::{GREEN, RED};
use macroquad::math::{IVec3, Vec2};
use macroquad::time::get_time;

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
    last_frame_time: f64,
    counted_frames: u32,
    fps: u32,
    pub sinks: bool,
    last_sink_time: f64,
    time_until_next_sink: f64,
}

impl GameHandler {
    pub fn new(paused: bool) -> GameHandler {
        let now = get_time();
        GameHandler {
            running: true,
            mouse_pos: Vec2::ZERO,
            last_mouse_pos: Vec2::ZERO,
            mouse_displacement: Vec2::ZERO,
            scroll: Scroll::Not,
            action: Action::None,
            paused,
            last_frame_time: now,
            counted_frames: 0,
            fps: 0,
            sinks: false,
            last_sink_time: now,
            time_until_next_sink: 1.0,
        }
    }

    pub fn events(&mut self, cam: &mut Camera) {
        self.mouse_pos = mouse_position_local();
        self.mouse_displacement = self.last_mouse_pos - self.mouse_pos;
        if self.mouse_displacement.x.abs() > 0.2 || self.mouse_displacement.y.abs() > 0.2 {self.mouse_displacement = Vec2::ZERO}
        self.last_mouse_pos = self.mouse_pos;

        if is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::Escape) {self.running = false};
        if is_key_pressed(KeyCode::Space) {self.paused = !self.paused};
        if is_mouse_button_down(MouseButton::Left) {cam.spherical_movement(self.mouse_displacement)};

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
                    if self.action == LeftRight {piece.try_move(field, IVec3::ZERO.with_y(if forwards {1} else {-1}));}
                    else if self.action == FrontBack {piece.try_move(field, IVec3::ZERO.with_x(if forwards {-1} else {1}));}
                    else if self.action == FlipFBAxis {piece.try_rotate(field, Axis::Y, !forwards);}
                    else if self.action == FlipLRAxis {piece.try_rotate(field, Axis::X, forwards);}
                
                } else if quadrant == 1 || quadrant == 2 {
                    if self.action == LeftRight {piece.try_move(field, IVec3::ZERO.with_x(if forwards {-1} else {1}));}
                    else if self.action == FrontBack {piece.try_move(field, IVec3::ZERO.with_y(if forwards {-1} else {1}));}
                    else if self.action == FlipFBAxis {piece.try_rotate(field, Axis::X, forwards);}
                    else if self.action == FlipLRAxis {piece.try_rotate(field, Axis::Y, forwards);}

                } else if quadrant == 3 || quadrant == 4 {
                    if self.action == LeftRight {piece.try_move(field, IVec3::ZERO.with_y(if forwards {-1} else {1}));}
                    else if self.action == FrontBack {piece.try_move(field, IVec3::ZERO.with_x(if forwards {1} else {-1}));}
                    else if self.action == FlipFBAxis {piece.try_rotate(field, Axis::Y, forwards);}
                    else if self.action == FlipLRAxis {piece.try_rotate(field, Axis::X, !forwards);}

                } else {
                    if self.action == LeftRight {piece.try_move(field, IVec3::ZERO.with_x(if forwards {1} else {-1}));}
                    else if self.action == FrontBack {piece.try_move(field, IVec3::ZERO.with_y(if forwards {1} else {-1}));}
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

    pub fn regulate_speed(&mut self) {
        
        let now = get_time();

        // Measure fps
        self.counted_frames += 1;
        if self.last_frame_time + 1.0 < now {
            self.last_frame_time = now;
            self.fps = self.counted_frames;
            self.counted_frames = 0;
        }

        // Defines sinks
        self.sinks = false;

        if self.last_sink_time + self.time_until_next_sink < now {
            self.last_sink_time = now;
            self.sinks = true;
            self.time_until_next_sink = self.get_time_until_next_sink();
        }   
    }

    pub fn get_fps(&self) -> u32 {self.fps}

    pub fn get_time_until_next_sink(&self) -> f64 {
        // Placeholder value since the game is too incomplete to balance it now
        1.0
    }
}