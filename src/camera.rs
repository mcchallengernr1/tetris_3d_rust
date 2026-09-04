use macroquad::color::{BLACK, WHITE};
use macroquad::math::Vec2;
use macroquad::math::Vec3;
use macroquad::time::get_time;
use macroquad::window::{clear_background, screen_height, screen_width};
use macroquad::prelude::draw_text;

use crate::cube::Cube;
use crate::utils::{Renderable, FaceNormal, FaceNormal::*};
use crate::field::Field;
use crate::piece::Piece;
use crate::point::*;
use crate::{CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z, CAMERA_RADIUS};

pub struct Camera {
    pos: Vec3,
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
    pub quadrant: i32,
    last_time: f64,
    counted_frames: u32,
    fps: u32,
}

impl Camera {
    pub fn new() -> Camera {
        let width = screen_width();
        let height = screen_height();
        let pos = Vec3::ZERO;
        let orbit_center_pos = Vec3::new(CELLS_IN_X as f32 / 2.0, CELLS_IN_Y as f32 / 2.0, CELLS_IN_Z as f32 / 2.0);
        let radius = CAMERA_RADIUS;
        let focal_length = 2.0;
        let inclination = std::f32::consts::PI / 2.0;
        let azimuth = 0.0;
        let polar_uv = Vec3::ZERO;
        let azimuth_uv = Vec3::ZERO;
        let spherical_mov_multiplier = width / 300.0;
        let quadrant = 0;
        let last_time = get_time();
        let counted_frames = 0;
        let fps = 0;

        Camera {pos, orbit_center_pos, radius, focal_length, width, height, inclination, azimuth, polar_uv, azimuth_uv, spherical_mov_multiplier, quadrant, last_time, counted_frames, fps }
    }
    
    pub fn project(&self, p: &Point) -> Vec2 {
        let unit_v = (p.pos - self.pos).normalize() * self.focal_length;

        Vec2::new(Vec3::dot(unit_v, self.azimuth_uv) * self.height + self.width / 2.0,
        Vec3::dot(unit_v, self.polar_uv) * self.height + self.height / 2.0)
    }

    pub fn update_internal_vars(&mut self) {
        self.width = screen_width();
        self.height = screen_height();
        
        self.counted_frames += 1;

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

    pub fn spherical_movement(&mut self, movement: Vec2) {
        self.azimuth += movement[0] * self.spherical_mov_multiplier;
        self.inclination += movement[1] * self.spherical_mov_multiplier;
        self.inclination = self.inclination.clamp(0.0, std::f32::consts::PI);
        if self.azimuth < 0.0 {self.azimuth += 2.0 * std::f32::consts::PI}
        if self.azimuth > std::f32::consts::PI * 2.0 {self.azimuth -= 2.0 * std::f32::consts::PI};

        self.quadrant = (4.0 * self.azimuth / std::f32::consts::PI).floor() as i32;
    }

    pub fn should_render_face(&self, face_drection: &FaceNormal, coordinate: Vec3) -> bool {
        match face_drection {
            XMinus => self.pos[0] < coordinate[0],
            XPlus => self.pos[0] > coordinate[0],
            YMinus => self.pos[1] < coordinate[1],
            YPlus => self.pos[1] > coordinate[1],
            ZMinus => self.pos[2] < coordinate[2],
            ZPlus => self.pos[2] > coordinate[2],
        }
    }

    pub fn clear_screen(&mut self) {
        clear_background(BLACK);
    }

    pub fn draw(&self, piece: &Piece, field: &Field) {
        let top_predraw = self.pos[2] < field.outline.t_xm.mid_pos[2];
        let grid_predraw = self.pos[2] > field.grid[0].mid_pos[2];
        let xm_ym_predraw = !(self.pos[0] < 0.0 || self.pos[1] < 0.0);
        let xp_ym_predraw = !(self.pos[0] > field.outline.xp_ym.mid_pos[0] || self.pos[1] < 0.0);
        let xm_yp_predraw = !(self.pos[0] < 0.0 || self.pos[1] > field.outline.xm_yp.mid_pos[1]);
        let xp_yp_predraw = !(self.pos[0] > field.outline.xp_yp.mid_pos[0] || self.pos[1] > field.outline.xp_yp.mid_pos[1]);

        // Predraw
        if top_predraw {
            field.outline.t_xm.draw(self);
            field.outline.t_xp.draw(self);
            field.outline.t_ym.draw(self);
            field.outline.t_yp.draw(self);
        }
        
        if grid_predraw {field.grid.iter().for_each(|s| s.draw(self));}
        
        if xm_ym_predraw {field.outline.xm_ym.draw(self);}
        if xm_yp_predraw {field.outline.xm_yp.draw(self);}
        if xp_ym_predraw {field.outline.xp_ym.draw(self);}
        if xp_yp_predraw {field.outline.xp_yp.draw(self);}

        // Draw Cubes
        let mut cubes: Vec<&Cube> = Vec::new();
        for cube in &field.cubes {
            cubes.push(cube);
        }

        for cube in &piece.cubes {
            cubes.push(cube);
        }

        cubes.sort_by(|c1, c2| c2.dist_to_pos(self.pos).total_cmp(&c1.dist_to_pos(self.pos)));

        cubes.iter().for_each(|c| c.draw(self));

        piece.axies.iter().for_each(|l| if l.on {l.draw(self);});

        // Postdraw
        if !top_predraw {
            field.outline.t_xm.draw(self);
            field.outline.t_xp.draw(self);
            field.outline.t_ym.draw(self);
            field.outline.t_yp.draw(self);
        }
        
        if !grid_predraw {field.grid.iter().for_each(|s| s.draw(self));}
        
        if !xm_ym_predraw {field.outline.xm_ym.draw(self);}
        if !xm_yp_predraw {field.outline.xm_yp.draw(self);}
        if !xp_ym_predraw {field.outline.xp_ym.draw(self);}
        if !xp_yp_predraw {field.outline.xp_yp.draw(self);}
    }

    pub fn display_text (&mut self, piece: &Piece) {
        let now = get_time();
        if self.last_time + 1.0 < now {
            self.last_time = now;
            self.fps = self.counted_frames;
            self.counted_frames = 0;
        }
        draw_text(format!("FPS: {0}\nquadrant: {1}\nPiece: {2}", self.fps, self.quadrant, piece.n), 10.0, 30.0, 40.0, WHITE);
    }
}