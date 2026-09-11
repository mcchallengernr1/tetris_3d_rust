use macroquad::color::Color;
use macroquad::math::IVec3;
use macroquad::rand::gen_range;

use crate::piece::Piece;
use crate::utils::{FaceNormal::*, in_field, _random_color};
use crate::{cube::Cube};
use crate::line::{Line};
use crate::utils::Axis;
use crate::{CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z};
use crate::utils::Movable;

pub struct Field {
    pub cubes: [[[Option<Cube>; CELLS_IN_X]; CELLS_IN_Y]; CELLS_IN_Z],
    pub outline: Outline,
    pub grid: Vec<Line>,
    pub occupancy_grid: [[[bool; CELLS_IN_X]; CELLS_IN_Y]; CELLS_IN_Z],
    _line_color: Color,
}

pub struct Outline {
    pub xm_ym: Line,
    pub xp_ym: Line,
    pub xm_yp: Line,
    pub xp_yp: Line,
    pub t_xm: Line,
    pub t_xp: Line,
    pub t_ym: Line,
    pub t_yp: Line,
}

impl Field {
    pub fn new(line_color: Color) -> Field {
        let mut grid = Vec::new();

        for i in 0..=CELLS_IN_X {
            grid.push(Line::new(IVec3::new(i as i32, 0,0), false, CELLS_IN_Y, Axis::Y, line_color, true))
        }
        for j in 0..=CELLS_IN_Y {
            grid.push(Line::new(IVec3::new(0, j as i32, 0), false, CELLS_IN_X, Axis::X, line_color, true));
        }

        Field { cubes: [[[None; CELLS_IN_X]; CELLS_IN_Y]; CELLS_IN_Z],
            outline: Outline {
            xm_ym: Line::new(IVec3::ZERO, false, CELLS_IN_Z, Axis::Z, line_color, true),
            xp_ym: Line::new(IVec3::new(CELLS_IN_X as i32, 0, 0), false, CELLS_IN_Z, Axis::Z, line_color, true),
            xm_yp: Line::new(IVec3::new(0, CELLS_IN_Y as i32, 0), false, CELLS_IN_Z, Axis::Z, line_color, true),
            xp_yp: Line::new(IVec3::new(CELLS_IN_X as i32, CELLS_IN_Y as i32, 0), false, CELLS_IN_Z, Axis::Z, line_color, true),
            t_xm: Line::new(IVec3::new(0, 0, CELLS_IN_Z as i32), false, CELLS_IN_Y, Axis::Y, line_color, true),
            t_xp: Line::new(IVec3::new(CELLS_IN_X as i32, 0, CELLS_IN_Z as i32), false, CELLS_IN_Y, Axis::Y, line_color, true),
            t_ym: Line::new(IVec3::new(0, 0, CELLS_IN_Z as i32), false, CELLS_IN_X, Axis::X, line_color, true),
            t_yp: Line::new(IVec3::new(0, CELLS_IN_Y as i32, CELLS_IN_Z as i32), false, CELLS_IN_X, Axis::X, line_color, true)},
            grid,
            occupancy_grid: [[[false; CELLS_IN_X]; CELLS_IN_Y]; CELLS_IN_Z],
            _line_color: line_color,
        }
    }

    pub fn taken_cube(&self, pos: IVec3) -> bool {
        self.occupancy_grid[pos[2] as usize][pos[1] as usize][pos[0] as usize]
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.add_cubes(Vec::from(piece.cubes));
    }

    fn add_cubes(&mut self, cubes: Vec<Cube>) {
        for c in cubes {
            self.occupancy_grid[c.pos[2] as usize][c.pos[1] as usize][c.pos[0] as usize] = true;
            self.cubes[c.pos[2] as usize][c.pos[1] as usize][c.pos[0] as usize] = Some(c);
        }

        self.disable_hidden_faces();
    }

    fn disable_hidden_faces(&mut self) {
        // for c in &mut self.cubes {
        //             // }
        for k in 0..CELLS_IN_Z {
            for j in 0..CELLS_IN_Y {
                for i in 0..CELLS_IN_X {
                    match &mut self.cubes[k][j][i] {
                        None => (),
                        Some(c) => {
                            c.faces[ZMinus.to_index()].on = if !in_field(c.pos.with_z(c.pos.z - 1)) {true} else {!self.occupancy_grid[(c.pos[2] - 1)as usize][c.pos[1] as usize][c.pos[0] as usize]};
                            c.faces[ZPlus.to_index()].on = if !in_field(c.pos.with_z(c.pos.z + 1)) {true} else {!self.occupancy_grid[(c.pos[2] + 1)as usize][c.pos[1] as usize][c.pos[0] as usize]};
                            c.faces[YMinus.to_index()].on = if !in_field(c.pos.with_y(c.pos.y - 1)) {true} else {!self.occupancy_grid[(c.pos[2])as usize][(c.pos[1] - 1) as usize][c.pos[0] as usize]};
                            c.faces[YPlus.to_index()].on = if !in_field(c.pos.with_y(c.pos.y + 1)) {true} else {!self.occupancy_grid[(c.pos[2])as usize][(c.pos[1] + 1) as usize][c.pos[0] as usize]};
                            c.faces[XMinus.to_index()].on = if !in_field(c.pos.with_x(c.pos.x - 1)) {true} else {!self.occupancy_grid[(c.pos[2])as usize][c.pos[1] as usize][(c.pos[0] - 1) as usize]};
                            c.faces[XPlus.to_index()].on = if !in_field(c.pos.with_x(c.pos.x + 1)) {true} else {!self.occupancy_grid[(c.pos[2])as usize][c.pos[1] as usize][(c.pos[0] + 1) as usize]};
                        }
                    }
                }
            }
        }
    }
    

    pub fn _fill_field_to_percent(&mut self, percent: u8, up_to_line: u8) {
        let mut cubes: Vec<Cube> = Vec::new();
        for k in 0..up_to_line as i32 {
            for j in 0..CELLS_IN_Y as i32 {
                for i in 0..CELLS_IN_X as i32 {
                    if gen_range(1, 100) <= percent {
                        // if !(i == 0 && j == 0) && !(i == CELLS_IN_X as i32 - 1 && k == 2) && !(i == CELLS_IN_X as i32 - 1 && k == 0) && !(i == CELLS_IN_X as i32 - 2 && k == 2){
                            cubes.push(Cube::new(IVec3::new(i, j, k), _random_color()));
                        // }
                    }
                }
            }
        }

        self.add_cubes(cubes);
    }

    pub fn try_line_clear(&mut self) -> usize {
        let mut lines_to_clear = Vec::new();
        for (i, line) in self.occupancy_grid.iter_mut().enumerate() {
            if line == &[[true; CELLS_IN_X]; CELLS_IN_Y] {
                lines_to_clear.push(i as i32);
            }
        }

        if !lines_to_clear.is_empty() {
            for line in lines_to_clear.iter().rev() {
                for k in *line as usize..(self.occupancy_grid.len() - 1) {
                    self.occupancy_grid[k] = self.occupancy_grid[k + 1];
                    self.cubes[k] = self.cubes[k + 1];
                    for j in 0..self.cubes[k].len() {
                        for i in 0..self.cubes[k][j].len() {
                            if let Some(c) = &mut self.cubes[k][j][i] {c.move_(IVec3::new(0, 0, -1))}
                        }
                    }
                }
                self.occupancy_grid[self.occupancy_grid.len() - 1] = [[false; CELLS_IN_X]; CELLS_IN_Y];
                self.cubes[self.cubes.len() - 1] = [[None; CELLS_IN_X]; CELLS_IN_Y];
            }
            
            self.disable_hidden_faces();
        }
        lines_to_clear.len()
    }
}