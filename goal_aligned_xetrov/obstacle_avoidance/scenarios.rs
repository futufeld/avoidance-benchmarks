use types::*;

use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;
use super::utilities::handler::*;
use super::common::vehicle::*;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

// Distance to which potential spreads from obstacles.
const POTENTIAL_SCALE: f64 = 10f64;

// Arrangement of vehicle and discs to be used in benchmarks.
pub struct Scenario {
    pub vehicle: Vehicle,
    pub discs: Vec<Box<HasSource>>
}

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&mut self) {
        let _ = self.vehicle.total_potential(&self.discs);
    }
}

impl Scenario {
    // Creates a scenario from the given vehicle and discs.
    pub fn new(vehicle: Vehicle, discs: Vec<Box<HasSource>>) -> Scenario {
        Scenario { vehicle: vehicle, discs: discs }
    }
}

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

// Helper function for creating random arrangements of discs and a vehicle.
fn scenario(num_discs: u32, dist_offset: f64) -> Scenario {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let orientation = vehicle.velocity.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    let mut discs: Vec<Box<HasSource>> = vec!();
    for _ in 0..num_discs {
        let ratio = 0.1f64 + 0.9f64 * random_unity();
        let radius = vehicle.potential_scale * ratio;

        let angle = 2f64 * PI * random_unity();
        let offset = radius + vehicle.potential_scale * dist_offset;
        let local_centre = Vec2D::polar(angle, offset);
        let centre = to_world.transform(local_centre);
        discs.push(Box::new(Disc::new(centre, radius)));
    }
    Scenario::new(vehicle, discs)
}

// Returns a randomly-generated scenario involving a vehicle positioned
// outside the loci of influence of a number of discs.
pub fn case1_scenario(num_discs: u32) -> Scenario {
    scenario(num_discs, 1f64 + random_unity())
}

// Returns a randomly-generated scenario involving a vehicle positioned
// inside the loci of influence of a number of discs.
pub fn case2_scenario(num_discs: u32) -> Scenario {
    scenario(num_discs, random_unity())
}
