use crate::{cube::Cube, field::Field, utils::Movable};
use macroquad::{color::Color, math::Vec3};
use crate::field::{CELLS_IN_X, CELLS_IN_Y, CELLS_IN_Z};
use crate::utils::{from_f32_to_i32, from_i32_to_f32, in_field, Dir};

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


// [[[255, 0, 0], [255, 51, 0], [255, 106, 0], [255, 157, 0], [255, 215, 0], [222, 235, 0], [188, 245, 0], [140, 255, 0], [89, 255, 0], [34, 255, 0], [0, 255, 17], [0, 255, 72], [0, 255, 123], [0, 255, 174], [0, 255, 229], [0, 229, 255], [0, 174, 255], [0, 123, 255], [0, 72, 255], [0, 17, 255], [34, 0, 255], [89, 0, 255], [140, 0, 255], [195, 0, 255], [247, 0, 255], [255, 0, 212], [255, 0, 157], [255, 0, 106], [255, 0, 51]]];

pub struct Piece {
    pub _n: usize,
    pub cubes: [Cube; 5],
    pub pos: [i32; 3],
}

impl Piece {
    pub fn new(n: usize) -> Piece {
        let pos = [CELLS_IN_X / 2, CELLS_IN_Y / 2, CELLS_IN_Z];
        // let pos = [CELLS_IN_X / 2, CELLS_IN_Y / 2, CELLS_IN_Z / 2];
        let cubes = PIECE_CONFIG[n].map(|cpos| Cube::new([cpos[0] + pos[0], cpos[1] + pos[1], cpos[2] + pos[2]], PIECE_COLOR[n]));
        Color::from_rgba(255, 255, 255, 255);
        Piece {
            _n: n,
            cubes,
            pos
        }
    }

    pub fn test_move(&mut self, field: &Field, mov: Vec3) {
        let mut valid = true;
        for cube in &self.cubes {
            let new_pos = from_f32_to_i32(from_i32_to_f32(cube.pos) + mov);
            if !in_field(new_pos) && field.taken_cube(new_pos) {
                valid = false;
            }
        }

        if valid {
            self.move_(mov)
        }
    }

    fn rotate(&mut self, axis: Dir, forwards: bool) {
        let mut cubes_new_pos = [[0; 3]; 5];

        for i in 0..self.cubes.len() {
            let relative_pos = [self.cubes[i].pos[0] - self.pos[0], self.cubes[i].pos[1] - self.pos[1], self.cubes[i].pos[2] - self.pos[2]];

            cubes_new_pos[i] = match axis {
                Dir::X => if forwards {[relative_pos[0], relative_pos[2], - relative_pos[1]]} else {[relative_pos[0], - relative_pos[2], relative_pos[1]]},
                Dir::Y => if forwards {[- relative_pos[2], relative_pos[1], relative_pos[0]]} else {[relative_pos[2], relative_pos[1], - relative_pos[0]]},
                Dir::Z => if forwards {[relative_pos[1], - relative_pos[0], relative_pos[2]]} else {[- relative_pos[1], relative_pos[0], relative_pos[2]]},
            };
        }
        
        
        
        
        //if field.empty
        //c.goto(
        //);

        //let mut x = 1;
        //let mut y = 2; 
        //let mut c: i32;
        //if forwards {c = x; x = y; y = - c}  x est x et y est y
        //else {c = x; x = - y; y = c}
        
    }

}

impl Movable for Piece {
    fn move_(&mut self, movement: Vec3) {
        for c in &mut self.cubes {
            c.move_(movement);
        }
    }
}