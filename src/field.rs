use macroquad::math::Vec3;

use crate::{cube::Cube, point::Point, segment::Segment};

const CELLS_IN_X: i32 = 9;
const CELLS_IN_Y: i32 = 9;
const CELLS_IN_Z: i32 = 20;

pub enum Dir {
    X,
    Y,
    Z
}

pub struct Line<'a> {
    points: Vec<Point>,
    segments: Vec<Segment<'a>>,
}

impl<'a> Line<'a> {
    pub fn new(pos: Vec3, length: u32, dir: Dir) -> Line<'a> {
        let points = Vec::new();
        let segments = Vec::new();

        for i in 0..=length {

        }

        // for k in 0..=CELLS_IN_Z {
        //     let z = k as f32 * C_S;
        //     let zp1 = k as f32 * C_S + 1.0;
            
        //     let p1 = Point::new(Vec3::new(0.0, 0.0, z));
        //     let p2 = Point::new(Vec3::new(0.0, 0.0, zp1));

        //     XmYm.add_point(p2);

        // }

        Line { points, segments }
    }
}

struct Outline<'a> {
    XmYm: Line<'a>,
    XpYm: Line<'a>,
    XmYp: Line<'a>,
    XpYp: Line<'a>,
    TXp: Line<'a>,
    TXm: Line<'a>,
    TYp: Line<'a>,
    TYm: Line<'a>,
}

impl<'a> Outline<'a> {
    fn new() -> Outline<'a> {
        Outline {
            XmYm: Line::new(Vec3::ZERO, CELLS_IN_Z as u32, Dir::Z),
            XpYm: Line::new(Vec3::new(CELLS_IN_X as f32, 0.0, 0.0), CELLS_IN_Z as u32, Dir::Z),
            XmYp: Line::new(Vec3::new(0.0, CELLS_IN_Y as f32, 0.0), CELLS_IN_Z as u32, Dir::Z),
            XpYp: Line::new(Vec3::new(CELLS_IN_X as f32, CELLS_IN_Y as f32, 0.0), CELLS_IN_Z as u32, Dir::Z),
            TXm: Line::new(Vec3::new(0.0, 0.0, CELLS_IN_Z as f32), CELLS_IN_Y as u32, Dir::Y),
            TXp: Line::new(Vec3::new(CELLS_IN_X as f32, 0.0, CELLS_IN_Z as f32), CELLS_IN_Y as u32, Dir::Y),
            TYm: Line::new(Vec3::new(0.0, 0.0, CELLS_IN_Z as f32), CELLS_IN_X as u32, Dir::X),
            TYp: Line::new(Vec3::new(0.0, CELLS_IN_Y as f32, CELLS_IN_Z as f32), CELLS_IN_X as u32, Dir::X),
        }
    }
}

pub struct Field<'a> {
    pub cubes: Vec<Cube>,
    outline: Outline<'a>,
}

impl<'a> Field<'a> {
    pub fn new() -> Field<'a> {
        Field { cubes: Vec::<Cube>::new(),
            outline: Outline::new(),
         }
    }
}