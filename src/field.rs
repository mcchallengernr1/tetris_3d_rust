use std::collections::HashMap;

use macroquad::color::Color;
use macroquad::math::Vec3;

use crate::C_S;
use crate::piece::Piece;
use crate::{cube::Cube};
use crate::line::{Line};
use crate::utils::Dir;

pub const CELLS_IN_X: i32 = 9;
pub const CELLS_IN_Y: i32 = 9;
pub const CELLS_IN_Z: i32 = 20;

pub struct Field<'a> {
    pub cubes: Vec<Cube>,
    pub outline: HashMap<&'a str, Line>,
    pub grid: Vec<Line>,
    occupancy_grid: [[[bool; CELLS_IN_X as usize]; CELLS_IN_Y as usize]; CELLS_IN_Z as usize],
    _line_color: Color,
}

impl<'a> Field<'a> {
    pub fn new(line_color: Color) -> Field<'a> {
        let mut grid = Vec::new();

        for i in 0..=CELLS_IN_X {
            grid.push(Line::new(Vec3::new(i as f32 * C_S, 0.0, 0.0), CELLS_IN_Y as u32, Dir::Y, line_color, true))
        }
        for j in 0..=CELLS_IN_Y {
            grid.push(Line::new(Vec3::new(0.0, j as f32 * C_S, 0.0), CELLS_IN_X as u32, Dir::X, line_color, true));
        }

        Field { cubes: Vec::<Cube>::new(),
            outline: HashMap::from([
            ("xm_ym", Line::new(Vec3::ZERO, CELLS_IN_Z as u32, Dir::Z, line_color, true)),
            ("xp_ym", Line::new(Vec3::new(CELLS_IN_X as f32, 0.0, 0.0), CELLS_IN_Z as u32, Dir::Z, line_color, true)),
            ("xm_yp", Line::new(Vec3::new(0.0, CELLS_IN_Y as f32, 0.0), CELLS_IN_Z as u32, Dir::Z, line_color, true)),
            ("xp_yp", Line::new(Vec3::new(CELLS_IN_X as f32, CELLS_IN_Y as f32, 0.0), CELLS_IN_Z as u32, Dir::Z, line_color, true)),
            ("t_xm", Line::new(Vec3::new(0.0, 0.0, CELLS_IN_Z as f32), CELLS_IN_Y as u32, Dir::Y, line_color, true)),
            ("t_xp", Line::new(Vec3::new(CELLS_IN_X as f32, 0.0, CELLS_IN_Z as f32), CELLS_IN_Y as u32, Dir::Y, line_color, true)),
            ("t_ym", Line::new(Vec3::new(0.0, 0.0, CELLS_IN_Z as f32), CELLS_IN_X as u32, Dir::X, line_color, true)),
            ("t_yp", Line::new(Vec3::new(0.0, CELLS_IN_Y as f32, CELLS_IN_Z as f32), CELLS_IN_X as u32, Dir::X, line_color, true))]),
            grid,
            occupancy_grid: [[[false; CELLS_IN_X as usize]; CELLS_IN_Y as usize]; CELLS_IN_Z as usize],
            _line_color: line_color,
        }
    }

    pub fn taken_cube(&self, pos: [i32; 3]) -> bool {
        println!("{:?}", pos);
        self.occupancy_grid[pos[2] as usize][pos[1] as usize][pos[0] as usize]
    }

    pub fn add_cubes(&mut self, piece: Piece) {
        for c in piece.cubes {
            self.occupancy_grid[c.pos[2] as usize][c.pos[1] as usize][c.pos[0] as usize] = true;
            self.cubes.push(c);
        }
    }
}