use crate::cube::Cube;
use macroquad::{color::{Color, GREEN}, math::Vec3};

const PIECE_CONFIG: [[[i32; 3]; 5]; 1]= [
    [[-2, 0, 0], [-1, 0, 0], [0, 0, 0], [1, 0, 0], [2, 0, 0]],
];

const PIECE_COLOR: [Color; 1] = [
    GREEN
];

pub struct Piece {
    pub n: usize,
    pub cubes: [Cube; 5],
    pub mid_pos: Vec3,
}

impl Piece {
    pub fn new(n: usize) -> Piece {
        let mid_pos = Vec3::ZERO;
        let cubes = PIECE_CONFIG[n].map(|pos| Cube::new(pos, PIECE_COLOR[n]));

        Piece {
            n,
            cubes,
            mid_pos
        }
    }
}