use macroquad::{color::{BLUE, Color, GREEN, RED, YELLOW}, shapes::{draw_line, draw_triangle}};

use crate::point::*;
use crate::camera::Camera;
use crate::utils::from_i32_to_f32;

pub enum FaceDirection {
    XPlus,
    XMinus,
    YPlus,
    YMinus,
    ZPlus,
    ZMinus
}

pub struct Face {
    points: [Point; 4],
    mid_pos: [f32; 3],
    direction: FaceDirection,
    color: Color,
    segment_color: Color,
}

impl Face {
    pub fn new(pos: [i32; 3], direction: FaceDirection, color: Color) -> Face {

        let mut mid_pos = from_i32_to_f32(pos);

        let poses = match direction {
            FaceDirection::XMinus => {mid_pos[1] += 0.5; mid_pos[2] += 0.5; [pos, [pos[0], pos[1], pos[2] + 1], [pos[0], pos[1] + 1, pos[2] + 1], [pos[0], pos[1] + 1, pos[2]]]},
            FaceDirection::XPlus => {mid_pos[0] += 1.0; mid_pos[1] += 0.5; mid_pos[2] += 0.5; [[pos[0] + 1, pos[1], pos[2]], [pos[0] + 1, pos[1] + 1, pos[2]], [pos[0] + 1, pos[1] + 1, pos[2] + 1], [pos[0] + 1, pos[1], pos[2] + 1]]},
            FaceDirection::YMinus => {mid_pos[0] += 0.5; mid_pos[2] += 0.5; [pos, [pos[0] + 1, pos[1], pos[2]], [pos[0] + 1, pos[1], pos[2] + 1], [pos[0], pos[1], pos[2] + 1]]},
            FaceDirection::YPlus => {mid_pos[0] += 0.5; mid_pos[1] += 1.0; mid_pos[2] += 0.5; [[pos[0], pos[1] + 1, pos[2]], [pos[0], pos[1] + 1, pos[2] + 1], [pos[0] + 1, pos[1] + 1, pos[2] + 1], [pos[0] + 1, pos[1] + 1, pos[2]]]},
            FaceDirection::ZMinus => {mid_pos[0] += 0.5; mid_pos[1] += 0.5; [pos, [pos[0], pos[1] + 1, pos[2]], [pos[0] + 1, pos[1] + 1, pos[2]], [pos[0] + 1, pos[1], pos[2]]]}
            FaceDirection::ZPlus => {mid_pos[0] += 0.5; mid_pos[1] += 0.5; mid_pos[2] += 1.0; [[pos[0], pos[1], pos[2] + 1], [pos[0] + 1, pos[1], pos[2] + 1], [pos[0] + 1, pos[1] + 1, pos[2] + 1], [pos[0], pos[1] + 1, pos[2] + 1]]}
        };

        let points = [Point::new(from_i32_to_f32(poses[0]), RED), Point::new(from_i32_to_f32(poses[1]), GREEN), Point::new(from_i32_to_f32(poses[2]), BLUE), Point::new(from_i32_to_f32(poses[3]), YELLOW)];

        let segment_color = Color::new(1.0, 1.0, 1.0, 1.0);
        Face {points, mid_pos, direction, color, segment_color}
    }

    pub fn draw(&self, cam: &Camera) {
        if cam.should_render_face(&self.direction, self.mid_pos) {
            let p1_pos = cam.project(&self.points[0]);
            let p2_pos = cam.project(&self.points[1]);
            let p3_pos = cam.project(&self.points[2]);
            let p4_pos = cam.project(&self.points[3]);
            draw_triangle(p1_pos, p2_pos, p3_pos, self.color);
            draw_triangle(p1_pos, p4_pos, p3_pos, self.color);
            draw_line(p1_pos[0], p1_pos[1], p2_pos[0], p2_pos[1], 1.0, self.segment_color);
            draw_line(p2_pos[0], p2_pos[1], p3_pos[0], p3_pos[1], 1.0, self.segment_color);
            draw_line(p3_pos[0], p3_pos[1], p4_pos[0], p4_pos[1], 1.0, self.segment_color);
            draw_line(p4_pos[0], p4_pos[1], p1_pos[0], p1_pos[1], 1.0, self.segment_color);
            // draw_circle(p1_pos[0], p1_pos[1], 10.0, self.points[0].color);
            // draw_circle(p2_pos[0], p2_pos[1], 10.0, self.points[1].color);
            // draw_circle(p3_pos[0], p3_pos[1], 10.0, self.points[2].color);
            // draw_circle(p4_pos[0], p4_pos[1], 10.0, self.points[3].color);
        }
    }
}