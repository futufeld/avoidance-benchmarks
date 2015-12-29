use types::*;

use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;
use super::utilities::handler::*;
use super::common::types::Frame;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

// Arrangement of vehicle and circles to be used in benchmarks.
pub struct Scenario {
    pub vehicle: Vehicle,
    pub circles: Vec<Circle>
}

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&mut self) {
        self.vehicle.frame.update_matrices();
        let _ = self.vehicle.obstacle_avoidance(&self.circles);
    }
}

impl Scenario {
    // Convenience function for creating scenarios.
    fn new(vehicle: Vehicle, circles: Vec<Circle>) -> Scenario {
        Scenario { vehicle: vehicle, circles: circles }
    }
}

// Returns a random f64 between 0 and 1 using the thread's random number
// generator.
fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

// Returns a vehicle with a semi-random position and orientation with the
// given length and width.
fn random_vehicle(length: f64, width: f64) -> Vehicle {
    let angle = 2f64 * PI * random_unity();
    let position = Vec2D::polar(angle, 100f64 * random_unity());
    let orientation = 2f64 * PI * random_unity();
    let vehicle = Frame::new(position, orientation);
    Vehicle::new(vehicle, length, width)
}

// Returns a circle with a semi-random centre determined by `x_scale`,
// `y_scale` and `y_offset`, which is then transformed by `transform`.
fn near_circle(x_scale: f64, y_scale: f64, y_offset: f64, transform: &Mat2D)
    -> Circle
{
    let local_x = x_scale * random_unity();
    let radius = y_scale * random_unity();
    let mut local_y = y_scale * y_offset + radius;
    if random_unity() < 0.5f64 { local_y = -local_y };

    let local_centre = Vec2D::new(local_x, local_y);
    let centre = transform.transform(local_centre);
    Circle::new(centre, radius)
}

// Returns a semi-random scenario involving `n` obstacles positioned with
// respect to `x_scale`, `y_scale` and `y_offset` (see `near_circle`).
fn scenario(n: u32, x_scale: f64, y_scale: f64, y_offset: f64) -> Scenario {
    let vehicle = random_vehicle(x_scale, y_scale);
    let to_world = vehicle.frame.to_world.clone();
    let f = |_| near_circle(x_scale, y_scale, y_offset, &to_world);
    let circles: Vec<Circle> = (0..n).map(f).collect();
    Scenario::new(vehicle, circles)
}

// Returns a randomly-generated arrangement of one vehicle and a number of
// circles. Each circle is guaranteed to lie outside the volume.
pub fn case1_scenario( num_circles: u32
                     , feeler_length: f64
                     , feeler_width: f64 ) -> Scenario {
    scenario(num_circles, 1f64 + random_unity(), feeler_length, feeler_width)
}

// Returns a randomly-generated arrangement of one vehicle and a number of
// circles. Each circle is guaranteed to intersect the vehicle's feeler volume.
pub fn case2_scenario( num_circles: u32
                     , feeler_length: f64
                     , feeler_width: f64 ) -> Scenario {
    let y_offset = 2f64 * (random_unity() - 0.5f64);
    scenario(num_circles, feeler_length, feeler_width, y_offset)
}
