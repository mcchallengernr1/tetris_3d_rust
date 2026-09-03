use std::collections::HashMap;

use macroquad::color::{Color, GRAY};
use macroquad::math::Vec3;
use macroquad::rand::gen_range;

use crate::C_S;
use crate::piece::Piece;
use crate::utils::{Direction::*, in_field};
use crate::{cube::Cube};
use crate::line::{Line};
use crate::utils::Dir;
use crate::{CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z};

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
        self.occupancy_grid[pos[2] as usize][pos[1] as usize][pos[0] as usize]
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.add_cubes(Vec::from(piece.cubes));
    }

    fn add_cubes(&mut self, cubes: Vec<Cube>) {
        for c in cubes {
            self.occupancy_grid[c.pos[2] as usize][c.pos[1] as usize][c.pos[0] as usize] = true;
            self.cubes.push(c);
        }

        self.disable_hidden_faces();
    }

    fn disable_hidden_faces(&mut self) {
        for c in &mut self.cubes {
            c.faces[ZMinus.to_index()].on = if !in_field([c.pos[0], c.pos[1], c.pos[2] - 1]) {true} else {!self.occupancy_grid[(c.pos[2] - 1)as usize][c.pos[1] as usize][c.pos[0] as usize]};
            c.faces[ZPlus.to_index()].on = if !in_field([c.pos[0], c.pos[1], c.pos[2] + 1]) {true} else {!self.occupancy_grid[(c.pos[2] + 1)as usize][c.pos[1] as usize][c.pos[0] as usize]};
            c.faces[YMinus.to_index()].on = if !in_field([c.pos[0], c.pos[1] - 1, c.pos[2]]) {true} else {!self.occupancy_grid[(c.pos[2])as usize][(c.pos[1] - 1) as usize][c.pos[0] as usize]};
            c.faces[YPlus.to_index()].on = if !in_field([c.pos[0], c.pos[1] + 1, c.pos[2]]) {true} else {!self.occupancy_grid[(c.pos[2])as usize][(c.pos[1] + 1) as usize][c.pos[0] as usize]};
            c.faces[XMinus.to_index()].on = if !in_field([c.pos[0] - 1, c.pos[1], c.pos[2]]) {true} else {!self.occupancy_grid[(c.pos[2])as usize][c.pos[1] as usize][(c.pos[0] - 1) as usize]};
            c.faces[XPlus.to_index()].on = if !in_field([c.pos[0] + 1, c.pos[1], c.pos[2]]) {true} else {!self.occupancy_grid[(c.pos[2])as usize][c.pos[1] as usize][(c.pos[0] + 1) as usize]};
        }
    }

    pub fn _fill_field(&mut self, pourcent: u8) {
        let mut cubes: Vec<Cube> = Vec::new();
        for k in 0..CELLS_IN_Z {
            for j in 0..CELLS_IN_Y {
                for i in 0..CELLS_IN_X {
                    if gen_range(1, 100) <= pourcent {
                        cubes.push(Cube::new([i, j, k], GRAY));
                    }
                }
            }
        }

        self.add_cubes(cubes);
    }
}