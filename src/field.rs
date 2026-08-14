use std::collections::HashMap;

use macroquad::math::Vec3;

use crate::C_S;
use crate::{cube::Cube};
use crate::line::{Line, Dir};

pub const CELLS_IN_X: i32 = 9;
pub const CELLS_IN_Y: i32 = 9;
pub const CELLS_IN_Z: i32 = 20;

pub struct Field<'a> {
    pub cubes: Vec<Cube>,
    pub outline: HashMap<&'a str, Line>,
    pub grid: Vec<Line>,
}

impl<'a> Field<'a> {
    pub fn new() -> Field<'a> {
        let mut grid = Vec::new();

        for i in 0..=CELLS_IN_X {
            grid.push(Line::new(Vec3::new(i as f32 * C_S, 0.0, 0.0), CELLS_IN_Y as u32, Dir::Y))
        }
        for j in 0..=CELLS_IN_Y {
            grid.push(Line::new(Vec3::new(0.0, j as f32 * C_S, 0.0), CELLS_IN_X as u32, Dir::X));
        }

        Field { cubes: Vec::<Cube>::new(),
            outline: HashMap::from([
            ("xm_ym", Line::new(Vec3::ZERO, CELLS_IN_Z as u32, Dir::Z)),
            ("xp_ym", Line::new(Vec3::new(CELLS_IN_X as f32, 0.0, 0.0), CELLS_IN_Z as u32, Dir::Z)),
            ("xm_yp", Line::new(Vec3::new(0.0, CELLS_IN_Y as f32, 0.0), CELLS_IN_Z as u32, Dir::Z)),
            ("xp_yp", Line::new(Vec3::new(CELLS_IN_X as f32, CELLS_IN_Y as f32, 0.0), CELLS_IN_Z as u32, Dir::Z)),
            ("t_xm", Line::new(Vec3::new(0.0, 0.0, CELLS_IN_Z as f32), CELLS_IN_Y as u32, Dir::Y)),
            ("t_xp", Line::new(Vec3::new(CELLS_IN_X as f32, 0.0, CELLS_IN_Z as f32), CELLS_IN_Y as u32, Dir::Y)),
            ("t_ym", Line::new(Vec3::new(0.0, 0.0, CELLS_IN_Z as f32), CELLS_IN_X as u32, Dir::X)),
            ("t_yp", Line::new(Vec3::new(0.0, CELLS_IN_Y as f32, CELLS_IN_Z as f32), CELLS_IN_X as u32, Dir::X))]),
            grid
        }
    }
}