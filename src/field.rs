use macroquad::math::Vec3;

use crate::{cube::Cube, point::Point, segment::Segment};
use crate::C_S;

const CELLS_IN_X: i32 = 9;
const CELLS_IN_Y: i32 = 9;
const CELLS_IN_Z: i32 = 20;

pub struct Field {
    pub cubes: Vec<Cube>,
    pub outline: Outline,
}

struct Outline {
    XmYm: Line,
    XpYm: Line,
    XmYp: Line,
    XpYp: Line,
}

struct Line {
    points: Vec<Point>,
    segments: Vec<Segment>,
}

impl Line {
    fn new() -> Line {
        Line { points: Vec::new(), segments: Vec::new() }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    fn add_segment(&mut self, s: Segment) {
        self.segments.push(s);
    }
}

impl Outline {
    fn new() -> Outline {
        let mut XmYm = Line::new();
        let mut XmYp = Line::new();
        let mut XpYm = Line::new();
        let mut XpYp = Line::new();

        for k in 0..=CELLS_IN_Z {
            let z = k as f32 * C_S;
            let zp1 = k as f32 * C_S + 1.0;
            
            let p1 = Point::new(Vec3::new(0.0, 0.0, z));
            let p2 = Point::new(Vec3::new(0.0, 0.0, zp1));

            XmYm.add_point(p2);

        }
        
        for p in XmYm.points {
            println!("{}", p.pos)
        }

        Outline {
            XmYm,
            XpYm,
            XmYp,
            XpYp,
        }
    }
}


impl Field {
    pub fn new() -> Field {
        Field { cubes: Vec::<Cube>::new(),
            outline: Outline::new(),
         }
    }
}