use macroquad::math::Vec2;

use crate::face::FaceDirection::*;
use crate::utils::*;
use crate::point::*;
use crate::face::*;

pub struct Camera {
    pos: [f32; 3],
    orbit_center_pos: [f32; 3],
    radius: f32,
    focal_length: f32,
    width: f32,
    height: f32,
    inclination: f32,
    azimuth: f32,
    polar_uv: [f32; 3],
    azimuth_uv: [f32; 3],
    spherical_mov_multiplier: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Camera {
        let pos = [0.0, 0.0, 0.0];
        let orbit_center_pos = [0.5, 0.5, 0.0];
        let radius = 10.0;
        let focal_length = 2.0;
        let inclination = std::f32::consts::PI / 2.0;
        let azimuth = 0.0;
        let polar_uv = [0.0, 0.0, 0.0];
        let azimuth_uv = [0.0, 0.0, 0.0];
        let spherical_mov_multiplier = 2.5;

        Camera {pos, orbit_center_pos, radius, focal_length, width, height, inclination, azimuth, polar_uv, azimuth_uv, spherical_mov_multiplier}
    }
    
    pub fn project(&self, p: &Point) -> Vec2 {
        let p_to_cam_v = add_v3d_to_v3d(p.pos, flip_v3d(self.pos));
        let unit_v = multiply_v_3d_by_s(direction(p_to_cam_v), self.focal_length);
        
        Vec2::new(dot_product(unit_v, self.azimuth_uv) * self.height + self.width / 2.0, 
        dot_product(unit_v, self.polar_uv) * self.height + self.height / 2.0)
    }

    pub fn update_internal_vars(&mut self) {
        let var1 = self.azimuth.cos();
        let var2 = self.inclination.cos();
        let var3 = self.azimuth.sin();
        let var4 = self.inclination.sin();
        self.polar_uv = [var2 * var1, var2 * var3, - var4];
        self.azimuth_uv = [- var3, var1, 0.0];

        self.pos = [
        self.orbit_center_pos[0] + self.radius * var4 * var1,
        self.orbit_center_pos[1] + self.radius * var4 * var3,
        self.orbit_center_pos[2] + self.radius * var2];
    }

    pub fn spherical_movement(&mut self, movement: [f32; 2]) {
        self.azimuth += movement[0] * self.spherical_mov_multiplier;
        self.inclination += movement[1] * self.spherical_mov_multiplier;
        if self.inclination < 0.0 {self.inclination = 0.0}
        if self.inclination > std::f32::consts::PI {self.inclination = std::f32::consts::PI}
        if self.azimuth < 0.0 {self.azimuth += 2.0 * std::f32::consts::PI}
        if self.azimuth > std::f32::consts::PI * 2.0 {self.azimuth -= 2.0 * std::f32::consts::PI};
    }

    pub fn should_render_face(&self, face_direction: &FaceDirection, coordinate: [f32; 3]) -> bool {
        match face_direction {
            XMinus => if self.pos[0] < coordinate[0] {true} else {false},
            XPlus => if self.pos[0] > coordinate[0] {true} else {false},
            YMinus => if self.pos[1] < coordinate[1] {true} else {false},
            YPlus => if self.pos[1] > coordinate[1] {true} else {false},
            ZMinus => if self.pos[2] < coordinate[2] {true} else {false},
            ZPlus => if self.pos[2] > coordinate[2] {true} else {false},
        }
    }
}