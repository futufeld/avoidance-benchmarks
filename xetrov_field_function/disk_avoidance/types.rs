use super::common::types::HasSource;
use super::linalg::matrix2d::Mat2D;
use super::linalg::vector2d::Vec2D;

// Defines transforms in and out of a space containing a disk.
pub struct Disk { pub to_world: Mat2D
                , pub to_local: Mat2D
                , pub radius:   f64 }

impl HasSource for Disk {
    // Returns the source corresponding to the given point.
    fn source(&self, v: Vec2D) -> Vec2D {
        let local = self.to_local.transform(v);
        let mag_sq = local.mag_sq();
        if mag_sq <= self.radius * self.radius { return v; }
        let source = local.mul(self.radius / mag_sq.sqrt());
        self.to_world.transform(source)
    }
}

impl Disk {
    // Creates a disk from a position and radius.
    pub fn new(pos: Vec2D, radius: f64) -> Disk {
        let to_world = Mat2D::translation(pos);
        let to_local = Mat2D::translation(pos.neg());
        Disk { to_world: to_world, to_local: to_local, radius: radius }
    }
}
