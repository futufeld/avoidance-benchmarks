use types::*;

use super::linalg::vector2d::*;
use super::utilities::handler::*;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

// Arrangement of sample point and disc to be used in benchmarks.
pub struct Scenario {
    pub sample: Vec2D,
    pub disc: Disc
}

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&self) {
        let _ = self.disc.source(self.sample);
    }
}

// Returns a random f64 between 0 and 1 using the thread's random number
// generator.
fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

// Helper function for creating an arrangement of a disc and point.
fn scenario(dist_offset: f64) -> Scenario {
    let sample_angle = 2f64 * PI * random_unity();
    let sample = Vec2D::new(sample_angle, 100f64 * random_unity());

    let radius = 5f64 + 5f64 * random_unity();
    let disc_angle = 2f64 * PI * random_unity();
    let disc_offset = Vec2D::polar(disc_angle, dist_offset * radius);
    let centre = sample.add(disc_offset);
    let disc = Disc::new(centre, radius);
    Scenario { sample: sample, disc: disc }
}

// Returns a randomly-generated scenario involving a sample point that lies
// within the radius of a disc.
pub fn case1_scenario() -> Scenario {
    scenario(random_unity())
}

// Returns a randomly-generated scenario involving a sample point that lies
// outside the radius of a disc.
pub fn case2_scenario() -> Scenario {
    scenario(1.1f64 + random_unity())
}
