use types::*;

use super::common::types::*;
use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;
use super::utilities::utilities::random_unity;

use std::f64::consts::PI;

// Returns a vehicle with semi-random position and velocity and a fixed
// potential scale.
fn random_vehicle() -> Vehicle {
    let angle = 2f64 * PI * random_unity();
    let position = Vec2D::polar(angle, 100f64 * random_unity());
    let velocity = Vec2D::polar(angle, 10f64);
    Vehicle::new(position, velocity, POTENTIAL_SCALE)
}

// Returns a line segment positioned semi-randomly with respect to
// `potential_scale` and `dist_offset` transformed by `to_world`.
fn near_wall(dist_offset: f64, potential_scale: f64, to_world: &Mat2D)
    -> Box<HasSource>
{
    let angle = 2f64 * PI * random_unity();
    let offset = dist_offset * potential_scale;
    let point = Vec2D::polar(angle, offset);
    let local_point1 = Vec2D::polar(angle + 0.5f64 * PI, 0.5f64);
    let local_point2 = Vec2D::polar(angle - 0.5f64 * PI, 0.5f64);

    let point1 = to_world.transform(point.add(local_point1));
    let point2 = to_world.transform(point.add(local_point2));
    Box::new(Segment::new(point1, point2))
}

// Helper function for creating an arrangement of a wall and vehicle.
fn scenario(num_walls: u32, dist_offset: f64) -> Scenario {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let orientation = vehicle.velocity.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    let f = |_| near_wall(dist_offset, POTENTIAL_SCALE, &to_world);
    let walls = (0..num_walls).map(f).collect();
    Scenario::new(vehicle, walls)
}

// Returns a randomly-generated scenario involving a vehicle positioned
// outside the loci of influence of a number of walls.
pub fn case1_scenario(num_discs: u32) -> Scenario {
    scenario(num_discs, 1f64 + random_unity())
}

// Returns a randomly-generated scenario involving a vehicle positioned
// inside the loci of influence of a number of walls.
pub fn case2_scenario(num_discs: u32) -> Scenario {
    scenario(num_discs, random_unity())
}
