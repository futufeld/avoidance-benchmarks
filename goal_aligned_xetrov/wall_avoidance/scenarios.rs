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

// Arrangement of sample point and disc to be used in benchmarks.
pub struct Scenario {
    pub vehicle: Vehicle,
    pub walls: Vec<Box<HasSource>>
}

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&mut self) {
        let _ = self.vehicle.total_potential(&self.walls);
    }
}

impl Scenario {
    // Creates a scenario from the given vehicle and walls.
    pub fn new(vehicle: Vehicle, walls: Vec<Box<HasSource>>) -> Scenario {
        Scenario { vehicle: vehicle, walls: walls }
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

// Helper function for creating an arrangement of a wall and vehicle.
fn scenario(num_walls: u32, dist_offset: f64) -> Scenario {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let orientation = vehicle.velocity.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    let mut walls: Vec<Box<HasSource>> = vec!();
    for _ in 0..num_walls {
        let angle = 2f64 * PI * random_unity();
        let offset = dist_offset * vehicle.potential_scale;
        let point = Vec2D::polar(angle, offset);
        let local_point1 = Vec2D::polar(angle + 0.5f64 * PI, 0.5f64);
        let local_point2 = Vec2D::polar(angle - 0.5f64 * PI, 0.5f64);

        let point1 = to_world.transform(point.add(local_point1));
        let point2 = to_world.transform(point.add(local_point2));
        walls.push(Box::new(Segment::new(point1, point2)));
    }
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
