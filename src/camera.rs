use macroquad::color::BLACK;
use macroquad::math::Vec2;
use macroquad::math::Vec3;
use macroquad::window::clear_background;

use crate::face::FaceDirection::*;
use crate::point::*;
use crate::face::*;

pub struct Camera {
    pub pos: Vec3,
    orbit_center_pos: Vec3,
    radius: f32,
    focal_length: f32,
    width: f32,
    height: f32,
    inclination: f32,
    azimuth: f32,
    polar_uv: Vec3,
    azimuth_uv: Vec3,
    spherical_mov_multiplier: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Camera {
        let pos = Vec3::ZERO;
        let orbit_center_pos = Vec3::ZERO;
        let radius = 50.0;
        let focal_length = 2.0;
        let inclination = std::f32::consts::PI / 2.0;
        let azimuth = 0.0;
        let polar_uv = Vec3::ZERO;
        let azimuth_uv = Vec3::ZERO;
        let spherical_mov_multiplier = width / 300.0;

        Camera {pos, orbit_center_pos, radius, focal_length, width, height, inclination, azimuth, polar_uv, azimuth_uv, spherical_mov_multiplier}
    }
    
    pub fn project(&self, p: &Point) -> Vec2 {
        let unit_v = (p.pos - self.pos).normalize() * self.focal_length;

        Vec2::new(Vec3::dot(unit_v, self.azimuth_uv) * self.height + self.width / 2.0,
        Vec3::dot(unit_v, self.polar_uv) * self.height + self.height / 2.0)
    }

    pub fn update_internal_vars(&mut self) {
        let az_cos = self.azimuth.cos();
        let incl_cos = self.inclination.cos();
        let az_sin = self.azimuth.sin();
        let incl_sin = self.inclination.sin();
        
        self.polar_uv = Vec3::new(incl_cos * az_cos, incl_cos * az_sin, - incl_sin);
        self.azimuth_uv = Vec3::new(- az_sin, az_cos, 0.0);


        self.pos = Vec3::new(
            self.orbit_center_pos[0] + self.radius * incl_sin * az_cos,
            self.orbit_center_pos[1] + self.radius * incl_sin * az_sin,
            self.orbit_center_pos[2] + self.radius * incl_cos);
    }

    pub fn spherical_movement(&mut self, movement: [f32; 2]) {
        self.azimuth += movement[0] * self.spherical_mov_multiplier;
        self.inclination += movement[1] * self.spherical_mov_multiplier;
        if self.inclination < 0.0 {self.inclination = 0.0}
        if self.inclination > std::f32::consts::PI {self.inclination = std::f32::consts::PI}
        if self.azimuth < 0.0 {self.azimuth += 2.0 * std::f32::consts::PI}
        if self.azimuth > std::f32::consts::PI * 2.0 {self.azimuth -= 2.0 * std::f32::consts::PI};
    }

    pub fn should_render_face(&self, face_direction: &FaceDirection, coordinate: Vec3) -> bool {
        match face_direction {
            XMinus => self.pos[0] < coordinate[0],
            XPlus => self.pos[0] > coordinate[0],
            YMinus => self.pos[1] < coordinate[1],
            YPlus => self.pos[1] > coordinate[1],
            ZMinus => self.pos[2] < coordinate[2],
            ZPlus => self.pos[2] > coordinate[2],
        }
    }

    pub fn clear_screen(&self) {
        clear_background(BLACK);
    }
}