use crate::line::Line;
use crate::utils::{from_f32_to_i32, Dir::*};
use crate::{cube::Cube, field::Field, utils::Movable};
use macroquad::{color::Color, math::Vec3, rand::gen_range};
use crate::field::{CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z};
use crate::utils::{Dir, add_i32_vec, from_i32_to_f32, in_field, sub_i32_vec};

const PIECE_CONFIG: [[[i32; 3]; 5]; 29]= [
    [[-2, 0, 0], [-1, 0, 0], [0, 0, 0], [1, 0, 0], [2, 0, 0]],
    [[-2, 0, 0], [-1, 0, 0], [0, 0, 0], [1, 0, 0], [1, 1, 0]],
    [[-2, 0, 0], [-1, 0, 0], [0, 0, 0], [0, 1, 0], [1, 1, 0]],
    [[-2, 0, 0], [-1, 0, 0], [0, 0, 0], [0, 1, 0], [1, 0, 0]],
    [[1, 0, 0], [-1, 0, 0], [0, 0, 0], [0, 1, 0], [1, 1, 0]],
    [[1, 0, 0], [-1, 0, 0], [0, 0, 0], [-1, 1, 0], [1, 1, 0]],
    [[-1, -1, 0], [-1, 0, 0], [0, -1, 0], [-1, 1, 0], [1, -1, 0]],
    [[0, 0, 0], [-1, 0, 0], [1, 0, 0], [-1, 1, 0], [1, -1, 0]],
    [[0, 0, 0], [-1, -1, 0], [-1, 1, 0], [-1, 0, 0], [1, 0, 0]],
    [[0, 0, 0], [-1, 0, 0], [0, -1, 0], [1, -1, 0], [-1, 1, 0]],
    [[0, 0, 0], [-1, 0, 0], [0, -1, 0], [1, 0, 0], [-1, 1, 0]],
    [[0, 0, 0], [-1, 0, 0], [0, -1, 0], [1, 0, 0], [0, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [1, 1, 0], [1, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, 1, 1]],
    [[0, 0, 0], [1, 0, 0], [0, 1, 0], [1, 0, 1], [0, 1, 1]],
    [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1], [1, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [0, 1, 0], [1, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [0, 1, 0], [-1, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [-1, 1, 0], [-1, 1, 1]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [1, 1, 0], [1, 1, 1]],
    [[0, 0, 0], [0, 1, 0], [-1, 1, 0], [1, 0, 0], [1, 0, 1]],
    [[0, 0, 0], [0, -1, 0], [-1, -1, 0], [1, 0, 0], [1, 0, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 0, 1], [1, 1, 0], [-1, 0, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 0, 1], [-1, 1, 0], [1, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [1, 1, 0], [-1, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [-1, 0, 0], [-1, 1, 0], [1, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1], [-1, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1], [-1, 0, 1]]];

const PIECE_COLOR: [Color; 29] = [
    Color::from_rgba(255, 0, 0, 255), 
    Color::from_rgba(255, 51, 0, 255), 
    Color::from_rgba(255, 106, 0, 255), 
    Color::from_rgba(255, 157, 0, 255), 
    Color::from_rgba(255, 215, 0, 255), 
    Color::from_rgba(222, 235, 0, 255), 
    Color::from_rgba(188, 245, 0, 255), 
    Color::from_rgba(140, 255, 0, 255), 
    Color::from_rgba(89, 255, 0, 255),
    Color::from_rgba(34, 255, 0, 255), 
    Color::from_rgba(0, 255, 17, 255), 
    Color::from_rgba(0, 255, 72, 255), 
    Color::from_rgba(0, 255, 123, 255), 
    Color::from_rgba(0, 255, 174, 255), 
    Color::from_rgba(0, 255, 229, 255), 
    Color::from_rgba(0, 229, 255, 255), 
    Color::from_rgba(0, 174, 255, 255),
    Color::from_rgba(0, 123, 255, 255), 
    Color::from_rgba(0, 72, 255, 255), 
    Color::from_rgba(0, 17, 255, 255), 
    Color::from_rgba(34, 0, 255, 255), 
    Color::from_rgba(89, 0, 255, 255), 
    Color::from_rgba(140, 0, 255, 255), 
    Color::from_rgba(195, 0, 255, 255), 
    Color::from_rgba(247, 0, 255, 255), 
    Color::from_rgba(255, 0, 212, 255), 
    Color::from_rgba(255, 0, 157, 255), 
    Color::from_rgba(255, 0, 106, 255), 
    Color::from_rgba(255, 0, 51, 255)];


pub struct Piece {
    pub _n: usize,
    pub cubes: [Cube; 5],
    pos: [i32; 3],
    pub axies: [Line; 3],
}

impl Piece {
    pub fn new(n: usize) -> Piece {
        let pos = [CELLS_IN_X / 2, CELLS_IN_Y / 2, CELLS_IN_Z - 2];

        let new_n: usize = if n > 29 {gen_range(0, 29)} else {n};

        let cubes = PIECE_CONFIG[new_n].map(|cpos| Cube::new(add_i32_vec(cpos, pos), PIECE_COLOR[new_n]));

        let axies = [
            Line::new(from_i32_to_f32(pos) + Vec3::new(-4.0, 0.5, 0.5), 9, X, Color::from_rgba(255, 0, 0, 255), false),
            Line::new(from_i32_to_f32(pos) + Vec3::new(0.5, -4.0, 0.5), 9, Y, Color::from_rgba(0, 255, 0, 255), false),
            Line::new(from_i32_to_f32(pos) + Vec3::new(0.5, 0.5, -4.0), 9, Z, Color::from_rgba(0, 0, 255, 255), false),
        ];
        
        Piece {
            _n: new_n,
            cubes,
            pos,
            axies
        }
    }

    pub fn test_move(&mut self, field: &Field, mov: [i32; 3]) -> bool {
        let mut valid = true;
        for cube in &self.cubes {
            let new_pos = add_i32_vec(cube.pos, mov);
            if !in_field(new_pos) || field.taken_cube(new_pos) {
                valid = false;
            }
        }

        if valid {self.move_(from_i32_to_f32(mov))};
        
        valid
    }

    pub fn turn_off_axies(&mut self) {
        self.axies.iter_mut().for_each(|l| l.on = false);
    }

    pub fn turn_on_axies(&mut self, i: usize) {
        if i < 3 {
            self.axies[i].on = true;
        }
    }

    pub fn test_rotate(&mut self, field: &Field, axis: Dir, forwards: bool) -> bool {
        // Compute and test for possible rotation and corrections
        let mut cubes_new_pos = [[0; 3]; 5];
        let mut valid = true;
        let mut kickback: [i32; 2] = [0, 0];

        for i in 0..self.cubes.len() {
            let rel_pos = sub_i32_vec(self.cubes[i].pos, self.pos);

            cubes_new_pos[i] = match axis {
                Dir::X => if forwards {[rel_pos[0], rel_pos[2], - rel_pos[1]]} else {[rel_pos[0], - rel_pos[2], rel_pos[1]]},
                Dir::Y => if forwards {[- rel_pos[2], rel_pos[1], rel_pos[0]]} else {[rel_pos[2], rel_pos[1], - rel_pos[0]]},
                Dir::Z => if forwards {[rel_pos[1], - rel_pos[0], rel_pos[2]]} else {[- rel_pos[1], rel_pos[0], rel_pos[2]]},
            };

            cubes_new_pos[i] = add_i32_vec(cubes_new_pos[i], self.pos);

            if cubes_new_pos[i][2] >= CELLS_IN_Z || cubes_new_pos[i][2] < 0 {valid = false}
            else if field.taken_cube(cubes_new_pos[i]) {valid = false}
            else if !in_field(cubes_new_pos[i]) {
                let new_x = if cubes_new_pos[i][0] < 0 {cubes_new_pos[i][0]} 
                    else if cubes_new_pos[i][0] - CELLS_IN_X < 0 {cubes_new_pos[i][0] - CELLS_IN_X}
                    else {0};
                let new_y = if cubes_new_pos[i][1] < 0 {cubes_new_pos[i][1]} 
                    else if cubes_new_pos[i][1] - CELLS_IN_Y < 0 {cubes_new_pos[i][1] - CELLS_IN_Y} 
                    else {0};

                kickback[0] = if new_x.abs() < kickback[0].abs() {kickback[0]} else {new_x};
                kickback[1] = if new_y.abs() < kickback[1].abs() {kickback[1]} else {new_y};
            }
        }

        // Check kickback
        if kickback != [0, 0] {
            for c in &mut cubes_new_pos {
                c[0] = c[0] + kickback[0];
                c[1] = c[1] + kickback[1];
            
                if field.taken_cube(*c) {
                    valid = false
                }
            }
        }

        // Move cubes for valid condition
        if valid {for i in 0..self.cubes.len() {self.cubes[i].goto(cubes_new_pos[i])}};
        
        valid
    }
}

impl Movable for Piece {
    fn move_(&mut self, movement: Vec3) {
        self.cubes.iter_mut().for_each(|c| c.move_(movement));
        self.axies.iter_mut().for_each(|l| l.move_(movement));
        self.pos = add_i32_vec(self.pos, from_f32_to_i32(movement))
        
    }
}