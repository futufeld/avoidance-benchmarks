use super::common::types::HasSource;
use super::linalg::matrix2d::*;
use super::linalg::vector2d::*;

// Defines transforms in and out of a space containing a line segment.
pub struct Segment { pub to_world: Mat2D
                   , pub to_local: Mat2D }

impl HasSource for Segment {
    // Returns the source corresponding to the given point.
    fn source(&self, v: Vec2D) -> Vec2D {
        let mut local = self.to_local.transform(v);
        if local.x < 0f64 { local.x = 0f64; }
        if local.x > 1f64 { local.x = 1f64; }
        local.y = 0f64;
        self.to_world.transform(local)
    }
}

impl Segment {
    // Creates a segment from two endpoints.
    pub fn new(point1: Vec2D, point2: Vec2D) -> Segment {
        let difference = point2.sub(point1);
        let orientation = difference.angle();
        let length = difference.mag();

        let scale = Vec2D::new(length, 1f64);
        let inverse_scale = Vec2D::new(1f64 / length, 1f64);
        let to_local = Mat2D::identity().shift(point1.neg())
                                        .turn(-orientation)
                                        .scale(inverse_scale);
        let to_world = Mat2D::identity().scale(scale)
                                        .turn(orientation)
                                        .shift(point1);
        Segment { to_world: to_world, to_local: to_local }
    }
}
