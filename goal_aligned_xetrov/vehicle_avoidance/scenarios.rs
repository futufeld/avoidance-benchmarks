use super::linalg::vector2d::*;
use super::common::types::*;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

// Returns a random f64 between 0 and 1 using the thread's random number
// generator.
fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

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

// Returns a semi-random scenario involving a single vehicle outside the loci
// of influence of `num_vehicles` other vehicles.
pub fn case1_scenario(num_vehicles: u32) -> Scenario {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let f = |_| near_vehicle(position, 1f64 + random_unity(), POTENTIAL_SCALE);
    let obstacles = (0..num_vehicles).map(f).collect();
    Scenario::new(vehicle, obstacles)
}

// Returns a semi-random scenario involving a single vehicle inside the loci
// of influence of `num_vehicles` other vehicles.
pub fn case2_scenario(num_vehicles: u32) -> Scenario {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let f = |_| near_vehicle(position, random_unity(), POTENTIAL_SCALE);
    let obstacles = (0..num_vehicles).map(f).collect();
    Scenario::new(vehicle, obstacles)
}
