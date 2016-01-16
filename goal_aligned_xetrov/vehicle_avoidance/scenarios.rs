use super::common::types::*;
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

// Returns a semi-random vehicle `dist_offset` units away from another.
fn near_vehicle(position: Vec2D, dist_offset: f64, potential_scale: f64)
    -> Box<HasSource>
{
    let angle1 = 2f64 * PI * random_unity();
    let offset1 = dist_offset * potential_scale;
    let future_position = position.add(Vec2D::polar(angle1, offset1));

    let angle2 = 2f64 * PI * random_unity();
    let offset2 = 0.25f64 * potential_scale * random_unity();
    let velocity = Vec2D::polar(angle2, offset2);
    let position = future_position.sub(velocity.mul(LOOK_AHEAD));

    Box::new(Vehicle::new(position, velocity, potential_scale))
}

// Returns a semi-random scenario involving a single vehicle inside or outside,
// depending on offset, the loci of influence of a number of other vehicles.
fn scenario(num_obstacles: u32, offset: f64) -> Box<Scenario> {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let f = |_| near_vehicle( position
                            , offset + random_unity()
                            , POTENTIAL_SCALE );
    let obstacles = (0..num_obstacles).map(f).collect();
    Box::new(Scenario::new(vehicle, obstacles))
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
