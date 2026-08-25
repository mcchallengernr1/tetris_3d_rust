use macroquad::{color::Color, prelude::{Vec2, Vec3}, shapes::{draw_line, draw_triangle}};


use crate::{point::*, utils::Movable};
use crate::{C_H_S,C_S};
use crate::camera::Camera;
use crate::utils::{Direction, Direction::*, Renderable,from_i32_to_f32};

pub struct Face {
    points: [Point; 4],
    mid_pos: Vec3,
    segment_color: Color,
    direction: Direction,
    pub on: bool,
    color: Color
}

impl Face {
    pub fn new(pos: [i32; 3], direction: Direction, color: Color) -> Face {

        let mut mid_pos = from_i32_to_f32(pos);

        let poses = match direction {
            XMinus => {mid_pos[1] += C_H_S; mid_pos[2] += C_H_S; [pos, [pos[0], pos[1], pos[2] + 1], [pos[0], pos[1] + 1, pos[2] + 1], [pos[0], pos[1] + 1, pos[2]]]},
            XPlus => {mid_pos[0] += C_S; mid_pos[1] += C_H_S; mid_pos[2] += C_H_S; [[pos[0] + 1, pos[1], pos[2]], [pos[0] + 1, pos[1] + 1, pos[2]], [pos[0] + 1, pos[1] + 1, pos[2] + 1], [pos[0] + 1, pos[1], pos[2] + 1]]},
            YMinus => {mid_pos[0] += C_H_S; mid_pos[2] += C_H_S; [pos, [pos[0] + 1, pos[1], pos[2]], [pos[0] + 1, pos[1], pos[2] + 1], [pos[0], pos[1], pos[2] + 1]]},
            YPlus => {mid_pos[0] += C_H_S; mid_pos[1] += C_S; mid_pos[2] += C_H_S; [[pos[0], pos[1] + 1, pos[2]], [pos[0], pos[1] + 1, pos[2] + 1], [pos[0] + 1, pos[1] + 1, pos[2] + 1], [pos[0] + 1, pos[1] + 1, pos[2]]]},
            ZMinus => {mid_pos[0] += C_H_S; mid_pos[1] += C_H_S; [pos, [pos[0], pos[1] + 1, pos[2]], [pos[0] + 1, pos[1] + 1, pos[2]], [pos[0] + 1, pos[1], pos[2]]]}
            ZPlus => {mid_pos[0] += C_H_S; mid_pos[1] += C_H_S; mid_pos[2] += C_S; [[pos[0], pos[1], pos[2] + 1], [pos[0] + 1, pos[1], pos[2] + 1], [pos[0] + 1, pos[1] + 1, pos[2] + 1], [pos[0], pos[1] + 1, pos[2] + 1]]}
        };

        let points = [Point::new(from_i32_to_f32(poses[0])), Point::new(from_i32_to_f32(poses[1])), Point::new(from_i32_to_f32(poses[2])), Point::new(from_i32_to_f32(poses[3]))];

        let segment_color = Color::new(1.0, 1.0, 1.0, 1.0);
        Face {points, mid_pos, segment_color, direction, on: true , color}
    }
}

impl Renderable for Face {
    fn draw(&self, cam: &Camera) {
        if self.on && cam.should_render_face(&self.direction, self.mid_pos) {
            let proj_p: Vec<Vec2> = self.points.iter()
                .map(|p| cam.project(p))
                .collect();

            draw_triangle(proj_p[0], proj_p[1], proj_p[2], self.color);
            draw_triangle(proj_p[0], proj_p[3], proj_p[2], self.color);

            draw_line(proj_p[0][0], proj_p[0][1], proj_p[1][0], proj_p[1][1], 1.0, self.segment_color);
            draw_line(proj_p[1][0], proj_p[1][1], proj_p[2][0], proj_p[2][1], 1.0, self.segment_color);
            draw_line(proj_p[2][0], proj_p[2][1], proj_p[3][0], proj_p[3][1], 1.0, self.segment_color);
            draw_line(proj_p[3][0], proj_p[3][1], proj_p[0][0], proj_p[0][1], 1.0, self.segment_color);
        }
    }
    
    fn dist_to_pos(&self, pos: Vec3) -> f32 {
        (self.mid_pos - pos).length()
    }
}

impl Movable for Face {
    fn move_(&mut self, movement: Vec3) {
        for p in &mut self.points {
            p.move_(movement)
        }
        self.mid_pos += movement;
    }
}