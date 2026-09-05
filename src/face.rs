use macroquad::{color::Color, math::IVec3, prelude::Vec3, shapes::{draw_line, draw_triangle}};


use crate::{point::*, utils::Movable};
use crate::{C_H_S,C_S};
use crate::camera::Camera;
use crate::utils::{FaceNormal, FaceNormal::*, Renderable};

pub struct Face {
    points: [Point; 4],
    mid_pos: Vec3,
    segment_color: Color,
    normal: FaceNormal,
    pub on: bool,
    color: Color
}

impl Face {
    pub fn new(pos: IVec3, normal: FaceNormal, color: Color) -> Face {

        let mut mid_pos = pos.as_vec3();

        let poses = match normal {
            XMinus => {mid_pos[1] += C_H_S; mid_pos[2] += C_H_S; [pos, IVec3::new(pos[0], pos[1], pos[2] + 1), IVec3::new(pos[0], pos[1] + 1, pos[2] + 1), IVec3::new(pos[0], pos[1] + 1, pos[2])]},
            XPlus => {mid_pos[0] += C_S; mid_pos[1] += C_H_S; mid_pos[2] += C_H_S; [IVec3::new(pos[0] + 1, pos[1], pos[2]), IVec3::new(pos[0] + 1, pos[1] + 1, pos[2]), IVec3::new(pos[0] + 1, pos[1] + 1, pos[2] + 1), IVec3::new(pos[0] + 1, pos[1], pos[2] + 1)]},
            YMinus => {mid_pos[0] += C_H_S; mid_pos[2] += C_H_S; [pos, IVec3::new(pos[0] + 1, pos[1], pos[2]), IVec3::new(pos[0] + 1, pos[1], pos[2] + 1), IVec3::new(pos[0], pos[1], pos[2] + 1)]},
            YPlus => {mid_pos[0] += C_H_S; mid_pos[1] += C_S; mid_pos[2] += C_H_S; [IVec3::new(pos[0], pos[1] + 1, pos[2]), IVec3::new(pos[0], pos[1] + 1, pos[2] + 1), IVec3::new(pos[0] + 1, pos[1] + 1, pos[2] + 1), IVec3::new(pos[0] + 1, pos[1] + 1, pos[2])]},
            ZMinus => {mid_pos[0] += C_H_S; mid_pos[1] += C_H_S; [pos, IVec3::new(pos[0], pos[1] + 1, pos[2]), IVec3::new(pos[0] + 1, pos[1] + 1, pos[2]), IVec3::new(pos[0] + 1, pos[1], pos[2])]}
            ZPlus => {mid_pos[0] += C_H_S; mid_pos[1] += C_H_S; mid_pos[2] += C_S; [IVec3::new(pos[0], pos[1], pos[2] + 1), IVec3::new(pos[0] + 1, pos[1], pos[2] + 1), IVec3::new(pos[0] + 1, pos[1] + 1, pos[2] + 1), IVec3::new(pos[0], pos[1] + 1, pos[2] + 1)]}
        };

        let points = [Point::new(poses[0].as_vec3()), Point::new(poses[1].as_vec3()), Point::new(poses[2].as_vec3()), Point::new(poses[3].as_vec3())];

        let segment_color = Color::new(1.0, 1.0, 1.0, 1.0);
        Face {points, mid_pos, segment_color, normal, on: true , color}
    }
}

impl Renderable for Face {
    fn draw(&self, cam: &Camera) {
        if self.on && cam.should_render_face(&self.normal, self.mid_pos) {
            let proj_p = [
                cam.project(self.points[0].pos),
                cam.project(self.points[1].pos),
                cam.project(self.points[2].pos),
                cam.project(self.points[3].pos)
            ];

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