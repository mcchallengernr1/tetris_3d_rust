use crate::cube::Cube;
use crate::piece::{PIECE_COLOR, PIECE_CONFIG};
use macroquad::math::{Vec3, IVec3};
use crate::CAMERA_RADIUS;

pub struct Showcase {
    pub cubes: Vec<Cube>,
    radius: f32,
    focal_length: f32,
    inclination: f32,
    azimuth: f32,
    polar_uv: Vec3,
    azimuth_uv: Vec3,
}

impl Showcase {
    pub fn new(n: usize) -> Showcase {
        let mut cubes = Vec::new();
        PIECE_CONFIG[n].iter().for_each(|pos| cubes.push(Cube::new(IVec3::from_array(*pos), PIECE_COLOR[n])));
    
        let radius = CAMERA_RADIUS;
        let focal_length = 2.0;
        let inclination = std::f32::consts::PI / 2.0;
        let azimuth = 0.0;
        let polar_uv = Vec3::ZERO;
        let azimuth_uv = Vec3::ZERO;


        Showcase {
            cubes,
            radius,
            focal_length,
            inclination,
            azimuth,
            polar_uv,
            azimuth_uv,
        }
    }

    pub fn draw(&mut self, x: i32, y: i32) {
        
    }
}