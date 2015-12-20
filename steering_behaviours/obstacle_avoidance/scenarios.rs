use types::*;

use super::linalg::vector2d::*;
use super::utilities::handler::*;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

// Arrangement of feeler volume and circles to be used in benchmarks.
pub struct Scenario {
    pub feeler: Feeler,
    pub circles: Vec<Circle>
}

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&mut self) {
        self.feeler.update_matrices();
        let _ = self.feeler.steering(&self.circles);
    }
}

impl Scenario {
    // Convenience function for creating scenarios.
    fn new(feeler: Feeler, circles: Vec<Circle>) -> Scenario {
        Scenario { feeler: feeler, circles: circles }
    }
}

// Returns a random f64 between 0 and 1 using the thread's random number
// generator.
fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

// Returns a feeler with a random position and orientation with the given
// length and width.
fn random_feeler(length: f64, width: f64) -> Feeler {
    let angle = 2f64 * PI * random_unity();
    let pos = Vec2D::new(angle, 100f64);
    let ori = random_unity() * 2f64 * PI;
    Feeler::new(pos, ori, length, width)
}

//
fn circle_near_feeler(feeler: &Feeler, offset: f64) -> Circle {
    let local_x = feeler.length * random_unity();
    let radius = feeler.width * random_unity();
    let mut local_y = feeler.width * offset + radius;
    if random_unity() < 0.5f64 { local_y = -local_y };

    let local_centre = Vec2D::new(local_x, local_y);
    let centre = feeler.to_world.transform(local_centre);
    Circle::new(centre, radius)
}

// Returns a randomly-generated arrangement of one feeler and a number of
// circles. Each circle is guaranteed to lie outside the volume.
pub fn case1_scenario(num_circles: u32, length: f64, width: f64) -> Scenario {
    let feeler = random_feeler(length, width);
    let mut circles = vec!();
    for _ in 0..num_circles {
        circles.push(circle_near_feeler(&feeler, 1f64 + random_unity()));
    }
    Scenario::new(feeler, circles)
}

// Returns a randomly-generated arrangement of one feeler and a number of
// circles. Each circle is guaranteed to intersect the feeler volume.
pub fn case2_scenario(num_circles: u32, length: f64, width: f64) -> Scenario {
    let feeler = random_feeler(length, width);
    let mut circles = vec!();
    for _ in 0..num_circles {
        let offset = 2f64 * (random_unity() - 0.5f64);
        circles.push(circle_near_feeler(&feeler, offset));
    }
    Scenario::new(feeler, circles)
}
