use crate::{cube::Cube, point::Point, segment::Segment};

const CELLS_IN_X: i32 = 9;
const CELLS_IN_Y: i32 = 9;
const CELLS_IN_Z: i32 = 20;

pub struct Field {
    pub cubes: Vec<Cube>,
    pub grid: Vec<Segment>,
}

struct Outline {
    BXmYm: Point,
    BXpYm: Point,
    BXmYp: Point,
    BXpYp: Point,
    TXmYm: Point,
    TXpYm: Point,
    TXmYp: Point,
    TXpYp: Point,
}
    
fn generate_grid() -> Vec<Segment> {
        let vec = Vec::<Segment>::new();

        for i in 0..=CELLS_IN_X {
            
        }


        vec
    }

impl Field {
    pub fn new() -> Field {
        Field { cubes: Vec::<Cube>::new(),
            grid : generate_grid()
         }
    }


}