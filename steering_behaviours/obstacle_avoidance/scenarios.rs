use types::*;

use super::linalg::vector2d::*;
use super::utilities::handler::*;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

// Arrangement of feeler volume and circles to be used in benchmarks.
pub struct Scenario {
    pub feeler: Feeler,
    pub circle: Circle
}

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&self) {
        let _ = self.feeler.obstacle_intersections(&self.circle);
    }
}

impl Scenario {
    // Convenience function for creating scenarios.
    fn new(feeler: Feeler, circle: Circle) -> Scenario {
        Scenario { feeler: feeler, circle: circle }
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

// Returns a randomly-generated arrangement of one feeler and a circle. The
// circles are guaranteed to lie outside the volume.
pub fn case1_scenario(length: f64, width: f64) -> Scenario {
    let feeler = random_feeler(length, width);

    let local_x = feeler.length * random_unity();
    let radius = feeler.width * random_unity();
    let mut local_y = feeler.width * 1.1f64 + radius;
    if random_unity() < 0.5f64 { local_y = -local_y };

    let local_centre = Vec2D::new(local_x, local_y);
    let centre = feeler.to_world.transform(local_centre);
    Scenario::new(feeler, Circle::new(centre, radius))
}

// Returns a randomly-generated arrangement of one feeler and a circle. Each
// circle is guaranteed to intersect one longitudinal edge of the feeler
// volume exactly twice.
pub fn case2_scenario(length: f64, width: f64) -> Scenario {
    let feeler = random_feeler(length, width);

    let mut x1 = 0.5f64 * random_unity();
    let mut x2 = 1f64 - 0.5f64 * random_unity();
    let mut xm = 0.5f64 * (x2 - x1);

    x1 *= feeler.length;
    x2 *= feeler.length;
    xm *= feeler.length;

    let point1 = Vec2D::new(x1, 0f64);
    let point2 = Vec2D::new(x1 + xm, xm);
    let point3 = Vec2D::new(x2, 0f64);

    let ma = (point2.y - point1.y) / (point2.x - point1.x);
    let mb = (point3.y - point2.y) / (point3.x - point2.x);

    let mut local_x = ma * mb * (point1.y - point3.y);
    local_x += mb * (point1.x + point2.x);
    local_x -= ma * (point2.x + point3.x);
    local_x /= 2f64 * (mb - ma);

    let mut local_y = -1f64 / ma;
    local_y *= local_x - 0.5f64 * (point1.x + point2.x);
    local_y += 0.5f64 * (point1.y + point2.y);

    local_y += feeler.width;
    if random_unity() <= 0.5f64 { local_y = -local_y };
    let local_centre = Vec2D::new(local_x, local_y);

    let centre = feeler.to_world.transform(local_centre);
    let radius = point2.sub(local_centre).mag();
    Scenario::new(feeler, Circle::new(centre, radius))
}

// Returns a randomly-generated arrangement of one feeler and a circle. The
// centre of each circle is guaranteed to lie inside the feeler volume.
pub fn case3_scenario(length: f64, width: f64) -> Scenario
{
    let feeler = random_feeler(length, width);

    let local_x = feeler.length * random_unity();
    let local_y = feeler.width * 2f64 * (random_unity() - 0.5f64);
    let local_centre = Vec2D::new(local_x, local_y);

    let centre = feeler.to_world.transform(local_centre);
    let radius = feeler.width * random_unity();
    Scenario::new(feeler, Circle::new(centre, radius))
}
