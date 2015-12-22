use types::*;

use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;
use super::utilities::handler::*;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

// Distance to which potential spreads from obstacles.
const POTENTIAL_SCALE: f64 = 10f64;

// Arrangement of sample point and disc to be used in benchmarks.
pub struct Scenario {
    pub vehicle: Vehicle,
    pub discs: Vec<Disc>
}

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&mut self) {
        let _ = self.vehicle.total_potential(&self.discs);
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

// Helper function for creating an arrangement of a disc and vehicle.
fn scenario(num_discs: u32, dist_offset: f64) -> Scenario {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let orientation = vehicle.velocity.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    let mut discs = vec!();
    for _ in 0..num_discs {
        let ratio = 0.1f64 + 0.9f64 * random_unity();
        let radius = vehicle.potential_scale * ratio;

        let angle = 2f64 * PI * random_unity();
        let offset = radius + vehicle.potential_scale * dist_offset;
        let local_centre = Vec2D::polar(angle, offset);
        let centre = to_world.transform(local_centre);
        discs.push(Disc::new(centre, radius));
    }
    Scenario { vehicle: vehicle, discs: discs }
}

// Returns a randomly-generated scenario involving a vehicle positioned
// outside the locus of influence of a disc.
pub fn case1_scenario(num_discs: u32) -> Scenario {
    scenario(num_discs, 1f64 + random_unity())
}

// Returns a randomly-generated scenario involving a vehicle positioned
// inside the locus of influence of a disc.
pub fn case2_scenario(num_discs: u32) -> Scenario {
    scenario(num_discs, random_unity())
}
