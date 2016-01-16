use types::*;

use super::common::types::*;
use super::linalg::matrix2d::Mat2D;
use super::linalg::vector2d::Vec2D;
use super::utilities::rng_utilities::*;
use super::utilities::types::{HasScenario, Obstacles};

// Returns a disc positioned semi-randomly with respect to `potential_scale`
// and `dist_offset` transformed by `to_world`.
fn near_disc(significant: bool, potential_scale: f64, transform: &Mat2D)
    -> Box<HasSource>
{
    let radius = potential_scale * random_margin();
    let mut dist_offset = random_margin();
    if !significant { dist_offset += 1f64; }
    let offset = radius + potential_scale * dist_offset;

    let local_centre = Vec2D::polar(random_tau(), offset);
    Box::new(Disc::new(transform.transform(local_centre), radius))
}

// Helper function for creating random arrangements of discs and a vehicle.
fn scenario(num_obstacles: u32, significant: bool) -> Box<Scenario> {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let orientation = vehicle.velocity.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    let f = |_| near_disc(significant, POTENTIAL_SCALE, &to_world);
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
