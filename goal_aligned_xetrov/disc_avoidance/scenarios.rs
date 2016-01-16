use types::*;

use super::common::types::*;
use super::linalg::matrix2d::*;
use super::linalg::vector2d::*;
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

// Returns a disc positioned semi-randomly with respect to `potential_scale`
// and `dist_offset` transformed by `to_world`.
fn near_disc(dist_offset: f64, potential_scale: f64, transform: &Mat2D)
    -> Box<HasSource>
{
    let ratio = 0.1f64 + 0.9f64 * random_unity();
    let radius = potential_scale * ratio;
    let angle = 2f64 * PI * random_unity();
    let offset = radius + potential_scale * dist_offset;
    let local_centre = Vec2D::polar(angle, offset);
    Box::new(Disc::new(transform.transform(local_centre), radius))
}

// Helper function for creating random arrangements of discs and a vehicle.
fn scenario(num_discs: u32, offset: f64) -> Box<Scenario> {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let orientation = vehicle.velocity.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    let f = |_| near_disc(offset + random_unity(), POTENTIAL_SCALE, &to_world);
    Box::new(Scenario::new(vehicle, (0..num_discs).map(f).collect()))
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
