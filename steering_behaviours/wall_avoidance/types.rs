use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;
use super::common::types::Frame;

// Used to indicate degenerate segment geometry.
#[derive(Debug)]
pub struct Degenerate;

// Captures the intersection between two lines.
enum LineLine {
    Coincident,
    Parallel,
    Intersect(f64,f64)
}

// Used to describe intersection between a feeler and wall.
#[derive(Copy, Clone)]
pub struct Interaction {
    force: Vec2D,
    dist: f64
}

impl Interaction {
    // Creates an interaction from the given values.
    fn new(force: Vec2D, dist: f64) -> Interaction {
        Interaction { force: force, dist: dist }
    }
}

// Defines a line segment.
pub struct Segment {
    pub point1: Vec2D,
    pub point2: Vec2D,
    pub length: f64,
    pub unit: Vec2D,
    pub normal: Vec2D
}

impl Segment {
    // Creates a line segment from the given points.
    pub fn new(point1: Vec2D, point2: Vec2D) -> Result<Segment, Degenerate> {
        let diff = point2.sub(point1);
        let length = diff.mag();
        let unit = diff.mul(1f64 / length);
        let result = Segment { point1: point1
                             , point2: point2
                             , length: length
                             , unit:   unit
                             , normal: unit.perp() };
        if length < EPSILON { Err(Degenerate) } else { Ok(result) }
    }

    // Transforms the two points of the segment using the given matrix.
    pub fn transform(&self, m: &Mat2D) -> Segment {
        Segment { point1: m.transform(self.point1)
                , point2: m.transform(self.point2)
                , length: self.length
                , unit: self.unit
                , normal: self.normal }
    }

    // Returns the intersection between this segment and another, considering
    // both as lines. Assumes the given geometry is not degenerate, which
    // should hold as long as the points inside the segments have not been
    // altered after the creation of the segment.
    fn line_intersection(&self, other: &Segment) -> LineLine {
        let pt1 = self.point1;
        let pt2 = self.point2;
        let pt3 = other.point1;
        let pt4 = other.point2;

        let mut a = (pt4.x - pt3.x) * (pt1.y - pt3.y);
        a -= (pt4.y - pt3.y) * (pt1.x - pt3.x);

        let mut b = (pt2.x - pt1.x) * (pt1.y - pt3.y);
        b -= (pt2.y - pt1.y) * (pt1.x - pt3.x);

        let mut d = (pt4.y - pt3.y) * (pt2.x - pt1.x);
        d -= (pt4.x - pt3.x) * (pt2.y - pt1.y);

        if d.abs() < EPSILON {
            if a.abs() < EPSILON && b.abs() < EPSILON {
                LineLine::Coincident
            } else {
                LineLine::Parallel
            }
        } else {
            LineLine::Intersect(a / d, b / d)
        }
    }

    // Returns the intersection of this segment and another, if the segments
    // are not parallel nor coincident. If the provided geometry is degenerate
    // this function will panic (should not occur, as verified by the tests).
    pub fn segment_intersection(&self, other: &Segment) -> Option<Vec2D> {
        match self.line_intersection(&other) {
            LineLine::Intersect(u1, u2) => {
                let u1_xsect = u1 >= 0f64 && u1 <= 1f64;
                let u2_xsect = u2 >= 0f64 && u2 <= 1f64;
                
                if !u1_xsect || !u2_xsect { return None };

                let diff = self.point2.sub(self.point1);
                Some(self.point1.add(diff.mul(u1)))
            },
            _ => None
        }
    }
}

// Vehicle with feelers.
pub struct Vehicle {
    pub frame: Frame,
    pub feelers: Vec<Segment>
}

impl Vehicle {
    // Creates a vehicle with the given values.
    pub fn new(frame: Frame, feelers: Vec<Segment>) -> Vehicle {
        Vehicle { frame: frame, feelers: feelers }
    }

    // Updates the matrices of the underlying frame.
    pub fn update(&mut self) {
        self.frame.update_matrices();
    }

    // Returns an interaction, if it exists, between a feeler and wall.
    fn wall_interaction(&self, feeler: &Segment, wall: &Segment)
        -> Option<Interaction>
    {
        match feeler.segment_intersection(wall) {
            Some(point) => {
                let mut normal = wall.normal;
                if normal.dot(feeler.unit) > 0f64 {
                    normal = normal.neg()
                }

                let dist = feeler.point1.sub(point).mag();
                let force = normal.mul(feeler.length - dist);
                Some(Interaction::new(force, dist))
            },
            None => None
        }
    }

    // Return a force intended to prevent collision between the vehicle and a
    // collection of walls.
    pub fn wall_avoidance(&self, walls: &Vec<Segment>) -> Option<Vec2D> {
        let mut nearest: Option<Interaction> = None;
        for local_feeler in self.feelers.iter() {
            let feeler = local_feeler.transform(&self.frame.to_world);
            for wall in walls.iter() {

                // Check if interaction is closer than known nearest.
                let interaction = self.wall_interaction(&feeler, &wall);
                if let Some(int) = interaction {
                    if let Some(near) = nearest {
                        if int.dist < near.dist { nearest = interaction }
                    } else {
                        nearest = interaction
                    }
                }
            }
        }
        if let Some(x) = nearest { Some(x.force) } else { None }
    }
}
