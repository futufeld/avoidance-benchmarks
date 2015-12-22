use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;

// Vehicle with position and orientation.
pub struct Vehicle {
    pub position: Vec2D,
    pub orientation: f64,
    pub to_world: Mat2D,
    pub to_local: Mat2D
}

impl Vehicle {
    // Creates a vehicle with the given position and orientation.
    pub fn new(pos: Vec2D, ori: f64) -> Vehicle {
        let to_local = Mat2D::translation(pos.neg()).turn(-ori);
        let to_world = Mat2D::rotation(ori).shift(pos);
        Vehicle { position: pos
                , orientation: ori
                , to_local: to_local
                , to_world: to_world }
    }

    // Updates the vehicle's matrices.
    pub fn update_matrices(&mut self) {
        let to_local = Mat2D::identity().shift(self.position.neg())
                                        .turn(-self.orientation);
        let to_world = Mat2D::identity().turn(self.orientation)
                                        .shift(self.position);
        self.to_local = to_local;
        self.to_world = to_world;
    }
}
