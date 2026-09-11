use crate::segment::Segment;
use crate::point::Point;
use crate::utils::{Axis, Movable, Renderable};
use macroquad::color::Color;
use macroquad::math::{Vec3, IVec3};
pub struct Line {
    pub segments: Vec<Segment>,
    pub mid_pos: Vec3,
    pub on: bool,
    _color: Color,
}

impl Line {
    pub fn new(pos: IVec3, centered: bool, length: usize, axis: Axis, color: Color, on: bool) -> Line {
        let mut segments = Vec::new();
        let mid_pos = match &axis {
            Axis::X => Vec3::new(pos.x as f32 + length as f32 / 2.0, if centered {pos.y as f32 + 0.5} else {pos.y as f32}, if centered {pos.z as f32 + 0.5} else {pos.z as f32}),
            Axis::Y => Vec3::new(if centered {pos.x as f32 + 0.5} else {pos.x as f32}, pos.y as f32 + length as f32 / 2.0, if centered {pos.z as f32 + 0.5} else {pos.z as f32}),
            Axis::Z => Vec3::new(if centered {pos.x as f32 + 0.5} else {pos.x as f32}, if centered {pos.y as f32 + 0.5} else {pos.y as f32}, pos.z as f32 + length as f32 / 2.0),
        };

        for i in 0..length as i32 {
            let pos = match axis {
                Axis::X => pos.with_x(i + pos[0]),
                Axis::Y => pos.with_y(i + pos[1]),
                Axis::Z => pos.with_z(i + pos[2]),
            };
            let mut point_pos = [pos.as_vec3(); 2];
            match &axis {
                Axis::X => {
                    point_pos[1].x += 1.0;
                    if centered {point_pos.iter_mut().for_each(|pos| {pos.y += 0.5; pos.z += 0.5});}
                }
                Axis::Y => {
                    point_pos[1].y += 1.0;
                    if centered {point_pos.iter_mut().for_each(|pos| {pos.x += 0.5; pos.z += 0.5});}
                }
                Axis::Z => {
                    point_pos[1].z += 1.0;
                    if centered {point_pos.iter_mut().for_each(|pos| {pos.x += 0.5; pos.y += 0.5});}
                }
            };
            segments.push(Segment::new(Point::new(point_pos[0]), Point::new(point_pos[1]), pos, color));
        }

        // for i in 0..length {
        //     let point_pos = match &axis {
        //         Axis::X => [Vec3::new(pos[0] + i as f32 * C_S, pos[1], pos[2]), Vec3::new(pos[0] + i as f32 * C_S + C_S, pos[1], pos[2])],
        //         Axis::Y => [Vec3::new(pos[0], pos[1] + i as f32 * C_S, pos[2]), Vec3::new(pos[0], pos[1] + i as f32 * C_S + C_S, pos[2])],
        //         Axis::Z => [Vec3::new(pos[0], pos[1], pos[2] + i as f32 * C_S), Vec3::new(pos[0], pos[1], pos[2] + i as f32 * C_S + C_S)]
        //     };
        //     segments.push(Segment::new(Point::new(point_pos[0]), Point::new(point_pos[1]), color));    
        // }
        Line { segments, mid_pos, on, _color: color }
    }

    pub fn change_color(&mut self, color: Color) {
        self._color = color;
        self.segments.iter_mut().for_each(|s| s.color = color);
    }


}

impl Renderable for Line {
    fn draw(&self, cam: &crate::camera::Camera) {
        self.segments.iter().for_each(|s| s.draw(cam));
    }
}

impl Movable for Line {
    fn move_(&mut self, mov: IVec3) {
        self.mid_pos += mov.as_vec3();
        self.segments.iter_mut().for_each(|s| s.move_(mov));
    }
}