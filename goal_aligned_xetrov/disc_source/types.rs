use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;

// Defines transforms in and out of a space containing a disc.
pub struct Disc {
    pub to_world: Mat2D,
    pub to_local: Mat2D,
    pub radius: f64
}

// Case identified when determining the source on a disc.
pub enum SourceResult {
    Case1(Vec2D),
    Case2(Vec2D)
}

impl Disc {
    // Creates a disc from a position and radius.
    pub fn new(pos: Vec2D, radius: f64) -> Disc {
        let to_world = Mat2D::identity().shift(pos);
        let to_local = Mat2D::identity().shift(pos.neg());
        Disc { to_world: to_world, to_local: to_local, radius: radius }
    }

    // Returns the source corresponding to `v`.
    pub fn source(&self, v: Vec2D) -> SourceResult {
        let local = self.to_local.transform(v);
        let mag = local.mag();
        let source = if mag <= self.radius { v } else {
            local.mul(self.radius / mag)
        };
        SourceResult::Case2(self.to_world.transform(source))
    }
}
