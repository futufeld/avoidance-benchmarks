use super::common::types::HasSource;
use super::linalg::matrix2d::Mat2D;
use super::linalg::vector2d::Vec2D;

// Defines transforms in and out of a space containing a disc.
pub struct Disc { pub to_world: Mat2D
                , pub to_local: Mat2D
                , pub radius:   f64 }

impl HasSource for Disc {
    // Returns the source corresponding to the given point.
    fn source(&self, v: Vec2D) -> Vec2D {
        let local = self.to_local.transform(v);
        let mag = local.mag();
        if mag <= self.radius { return v; }
        let source = local.mul(self.radius / mag);
        self.to_world.transform(source)
    }
}

impl Disc {
    // Creates a disc from a position and radius.
    pub fn new(pos: Vec2D, radius: f64) -> Disc {
        let to_world = Mat2D::translation(pos);
        let to_local = Mat2D::translation(pos.neg());
        Disc { to_world: to_world, to_local: to_local, radius: radius }
    }
}
