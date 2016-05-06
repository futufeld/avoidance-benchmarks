use types::*;

use super::common::types::*;
use super::linalg::vector2d::Vec2D;
use super::linalg::matrix2d::Mat2D;
use super::utilities::rng_utilities::*;
use super::utilities::types::{HasScenario, Obstacles};

use std::f64::consts::PI;

// Returns a line segment positioned semi-randomly.
fn near_wall(significant: bool, potential_scale: f64, to_world: &Mat2D)
    -> Box<HasSource>
{
    let angle = random_tau();
    let mut dist_offset = random_margin();
    if !significant { dist_offset += 1f64; }
    let offset = dist_offset * potential_scale;
    let point = Vec2D::polar(angle, offset);

    let local_point1 = Vec2D::polar(angle + 0.5f64 * PI, 0.5f64);
    let local_point2 = Vec2D::polar(angle - 0.5f64 * PI, 0.5f64);

    let point1 = to_world.transform(point.add(local_point1));
    let point2 = to_world.transform(point.add(local_point2));
    Box::new(Segment::new(point1, point2))
}

// Helper function for creating an arrangement of a wall and vehicle.
fn scenario(num_obstacles: u32, significant: bool) -> Box<Scenario> {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let orientation = vehicle.velocity.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    let f = |_| near_wall(significant, POTENTIAL_SCALE, &to_world);
    Box::new(Scenario::new(vehicle, (0..num_obstacles).map(f).collect()))
}

// Returns a scenario with the given configuration of obstacles. Returns none
// if it is not possible to create the given scenario.
pub fn scenario_with_obstacles(obstacles: &Obstacles)
    -> Option<Box<HasScenario>>
{
    match obstacles.details() {
        (num_obs, 0u32) => Some(scenario(num_obs, false)),
        (0u32, num_obs) => Some(scenario(num_obs, true)),
        _ => None
    }
}
