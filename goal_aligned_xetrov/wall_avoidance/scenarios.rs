use types::*;

use super::common::types::*;
use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;
use super::utilities::handler::{HasScenario, Obstacles};
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
fn scenario(num_walls: u32, offset: f64) -> Box<Scenario> {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let orientation = vehicle.velocity.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    let f = |_| near_wall(offset + random_unity(), POTENTIAL_SCALE, &to_world);
    Box::new(Scenario::new(vehicle, (0..num_walls).map(f).collect()))
}

// Returns a scenario with the given configuration of obstacles. Returns none
// if it is not possible to create the given scenario.
pub fn scenario_with_obstacles(obstacles: &Obstacles)
    -> Option<Box<HasScenario>>
{
    match obstacles.details() {
        (num_obs, 0u32) => Some(scenario(num_obs, 1f64)),
        (0u32, num_obs) => Some(scenario(num_obs, 0f64)),
        _ => None
    }
}
