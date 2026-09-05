use std::array;
use crate::line::Line;
use crate::utils::{Axis::*, Movable};
use crate::{cube::Cube, field::Field};
use macroquad::{color::Color, math::{Vec3, IVec3}, rand::gen_range};
use crate::{AXIS_LENGTH, CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z};
use crate::utils::{Axis, in_field};

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
    pub n: usize,
    pub cubes: [Cube; PIECE_CONFIG[0].len()],
    pos: IVec3,
    pub axies: [Line; 3],
}

impl Piece {
    pub fn new(n: usize) -> Piece {
        let pos = IVec3::new((CELLS_IN_X / 2) as i32, (CELLS_IN_Y / 2) as i32, (CELLS_IN_Z - 2) as i32);

        let cubes = PIECE_CONFIG[n].map(|cpos| Cube::new(IVec3::from_array(cpos) + pos, PIECE_COLOR[n]));

        let axies = [
            Line::new(pos.as_vec3() + Vec3::new(-4.0, 0.5, 0.5), AXIS_LENGTH, X, Color::from_rgba(255, 0, 0, 255), false),
            Line::new(pos.as_vec3() + Vec3::new(0.5, -4.0, 0.5), AXIS_LENGTH, Y, Color::from_rgba(0, 255, 0, 255), false),
            Line::new(pos.as_vec3() + Vec3::new(0.5, 0.5, -4.0), AXIS_LENGTH, Z, Color::from_rgba(0, 0, 255, 255), false),
        ];
        
        Piece {
            n,
            cubes,
            pos,
            axies
        }
    }

    pub fn new_random() -> Piece {
        Piece::new(gen_range(0, PIECE_CONFIG.len()))
    }

    pub fn try_move(&mut self, field: &Field, mov: IVec3) -> bool {
        let mut valid = true;
        for cube in &self.cubes {
            let new_pos = cube.pos + mov;
            if !in_field(new_pos) || field.taken_cube(new_pos) {
                valid = false;
            }
        }

        if valid {self.move_(mov)};
        
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

    pub fn try_rotate(&mut self, field: &Field, axis: Axis, forwards: bool) -> bool {
        // Compute and test for possible rotation and corrections

        let mut cubes_new_pos: [IVec3; 5] = array::from_fn(|i| {
            self.turn_cube_pos(self.cubes[i].pos - self.pos, &axis, forwards)
        });

        let mut valid = true;
        let mut kickback: [i32; 2] = [0, 0];

        for c in cubes_new_pos {
            if c[2] >= CELLS_IN_Z as i32|| c[2] < 0 {valid = false}
            else if !in_field(c) {
                let x: i32 = if c[0] < 0 {- c[0]} else if c[0] >= CELLS_IN_X as i32 {CELLS_IN_X as i32 - 1 - c[0]} else {0};
                let y: i32 = if c[1] < 0 {- c[1]} else if c[1] >= CELLS_IN_Y as i32 {CELLS_IN_Y as i32 - 1 - c[1]} else {0};
                
                if x.abs() > kickback[0].abs() {kickback[0] = x};
                if y.abs() > kickback[1].abs() {kickback[1] = y};
            }
            else if field.taken_cube(c) {valid = false}
        }

        // Check kickback
        if valid && kickback != [0, 0] {
            for c in &mut cubes_new_pos {
                c[0] += kickback[0];
                c[1] += kickback[1];
            
                if field.taken_cube(*c) {
                    valid = false
                }
            }
        }

        // Move cubes for valid condition
        if valid {
            self.move_(IVec3::new(kickback[0], kickback[1], 0));

            self.cubes
                .iter_mut()
                .enumerate()
                .for_each(|(i, c)| c.goto(cubes_new_pos[i]));
        }

        valid
    }

    fn turn_cube_pos(&self, pos: IVec3, axis: &Axis, forwards: bool) -> IVec3 {
        self.pos + match axis {
                Axis::X => if forwards {IVec3::new(pos[0], - pos[2], pos[1])} else {IVec3::new(pos[0], pos[2], - pos[1])},
                Axis::Y => if forwards {IVec3::new(pos[2], pos[1], - pos[0])} else {IVec3::new(- pos[2], pos[1], pos[0])},
                Axis::Z => if forwards {IVec3::new(- pos[1], pos[0], pos[2])} else {IVec3::new(pos[1], - pos[0], pos[2])},
            }
    }

    fn move_(&mut self, mov: IVec3) {
        self.pos += mov;
        self.cubes.iter_mut().for_each(|c| c.move_(mov));
        self.axies.iter_mut().for_each(|l| l.move_(mov.as_vec3()));
    }
}