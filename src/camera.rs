use macroquad::color::{BLACK, WHITE};
use macroquad::math::Vec2;
use macroquad::math::Vec3;
use macroquad::window::{clear_background, screen_height, screen_width};
use macroquad::prelude::draw_text;

use crate::utils::in_field;
use crate::utils::{FaceNormal::{self, *}, Renderable};
use crate::field::Field;
use crate::piece::Piece;
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

        Camera {pos, orbit_center_pos, radius, focal_length, width, height, inclination, azimuth, polar_uv, azimuth_uv, spherical_mov_multiplier, quadrant }
    }
    
    pub fn project(&self, pos: Vec3) -> Vec2 {
        let unit_v = (pos - self.pos).normalize() * self.focal_length;

        Vec2::new(Vec3::dot(unit_v, self.azimuth_uv) * self.height + self.width / 2.0,
        Vec3::dot(unit_v, self.polar_uv) * self.height + self.height / 2.0)
    }

    pub fn update_internal_vars(&mut self) {
        self.width = screen_width();
        self.height = screen_height();

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

    pub fn draw(&self, piece: &Piece, field: &mut Field) {
        let axis_i = piece.get_active_axis_index();

        let top_predraw = self.pos[2] < field.outline.t_xm.mid_pos[2];
        let grid_predraw = self.pos[2] > field.grid[0].mid_pos[2];
        let xm_ym_predraw = !(self.pos[0] < 0.0 || self.pos[1] < 0.0);
        let xp_ym_predraw = !(self.pos[0] > field.outline.xp_ym.mid_pos[0] || self.pos[1] < 0.0);
        let xm_yp_predraw = !(self.pos[0] < 0.0 || self.pos[1] > field.outline.xm_yp.mid_pos[1]);
        let xp_yp_predraw = !(self.pos[0] > field.outline.xp_yp.mid_pos[0] || self.pos[1] > field.outline.xp_yp.mid_pos[1]);        

        // Predraw
        
        if let Some(i) = axis_i {
            piece.axies[i].segments.iter().for_each(|s| if !in_field(s.pos) {
                if s.pos[0] >= CELLS_IN_X as i32 && self.pos[0] <= CELLS_IN_X as f32 {s.draw(self);}
                if s.pos[0] <= 0 && self.pos[0] >= 0.0 {s.draw(self);}
                if s.pos[1] >= CELLS_IN_Y as i32 && self.pos[1] <= CELLS_IN_Y as f32 {s.draw(self);}
                if s.pos[1] <= 0 && self.pos[1] >= 0.0 {s.draw(self);}
                if s.pos[2] >= CELLS_IN_Z as i32 && top_predraw {s.draw(self);}
                if s.pos[2] <= 0 && grid_predraw {s.draw(self);}
            })
        };

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
        piece.cubes.iter().for_each(|c| field.cubes[c.pos[2] as usize][c.pos[1] as usize][c.pos[0] as usize] = Some(*c));

        let k_switch =  if self.pos[2] < 0.5 {0} else if self.pos[2] > CELLS_IN_Z as f32 - 0.5 { CELLS_IN_Z } else {(self.pos[2] - 0.5).floor() as usize};
        let j_switch = if self.pos[1] < 0.0 {0} else if self.pos[1] > CELLS_IN_Y as f32 { CELLS_IN_Y } else {(self.pos[1]).floor() as usize};
        let i_switch = if self.pos[0] < 0.0 {0} else if self.pos[0] > CELLS_IN_X as f32 { CELLS_IN_X } else {(self.pos[0]).floor() as usize};

        for k_ in 0..CELLS_IN_Z {
            let k = if k_ < k_switch { k_ } else { CELLS_IN_Z - 1 - k_ + k_switch };
            for j_ in 0..CELLS_IN_Y {
                let j = if j_ < j_switch { j_ } else { CELLS_IN_Y - 1 - j_ + j_switch };
                for i_ in 0..CELLS_IN_X {
                    let i = if i_ < i_switch { i_ } else { CELLS_IN_X - 1 - i_ + i_switch };
                    match &field.cubes[k][j][i] {
                        None => if let Some(i_) = axis_i {
                            piece.axies[i_].segments.iter().for_each(|s| 
                                if s.pos.x == i as i32 && s.pos.y == j as i32 && s.pos.z == k as i32 {s.draw(self);});
                        },
                        Some(c) => {
                            c.draw(self);
                        } 
                    }
                }
            }
        }
        
        piece.cubes.iter().for_each(|c| field.cubes[c.pos[2] as usize][c.pos[1] as usize][c.pos[0] as usize] = None);
        // piece.cubes.iter().for_each(|c| c.draw(self));
        // piece.axies.iter().for_each(|l| if l.on {l.draw(self);});

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

        if let Some(i) = axis_i {
            piece.axies[i].segments.iter().for_each(|s| if !in_field(s.pos) {
                if s.pos[0] >= CELLS_IN_X as i32 && self.pos[0] >= CELLS_IN_X as f32 {s.draw(self);}
                if s.pos[0] <= 0 && self.pos[0] <= 0.0 {s.draw(self);}
                if s.pos[1] >= CELLS_IN_Y as i32 && self.pos[1] >= CELLS_IN_Y as f32 {s.draw(self);}
                if s.pos[1] < 0 && self.pos[1] <= 0.0 {s.draw(self);}
                if s.pos[2] >= CELLS_IN_Z as i32 && !top_predraw {s.draw(self);}
                if s.pos[2] <= 0 && !grid_predraw {s.draw(self);}
            })
        };

    }

    pub fn display_text (&mut self, piece: &Piece, fps: u32) {
        draw_text(format!("FPS: {0}  quadrant: {1}  Piece: {2}", fps, self.quadrant, piece.n), 10.0, 30.0, 40.0, WHITE);
    }
}