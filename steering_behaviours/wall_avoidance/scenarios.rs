use types::*;

use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;
use super::utilities::handler::*;
use super::common::types::Frame;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

// Arrangement of vehicle and line segment obstacles.
pub struct Scenario {
    pub vehicle: Vehicle,
    pub walls: Vec<Segment>
}

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&mut self) {
        self.vehicle.update();
        let _ = self.vehicle.wall_avoidance(&self.walls);
    }
}

impl Scenario {
    // Creates a scenario involving a vehicle with feelers and wall segments.
    pub fn new(vehicle: Vehicle, walls: Vec<Segment>) -> Scenario {
        Scenario { vehicle: vehicle, walls: walls }
    }
}

// Defines feeler arrangements.
#[derive(Copy, Clone)]
pub enum FeelerShape { Spear, Fork, Trident }

// Returns a random f64 between 0 and 1 using the thread's random number
// generator.
fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

// Returns a frame with semi-random position and orientation.
fn random_frame() -> Frame {
    let angle = 2f64 * PI * random_unity();
    let position = Vec2D::polar(angle, 100f64 * random_unity());
    Frame::new(position, 2f64 * PI * random_unity())
}

// Returns a feeler that extends ahead of a vehicle.
fn feeler_centre() -> Segment {
    Segment::new(Vec2D::new(2f64, 0f64), Vec2D::new(12f64, 0f64)).unwrap()
}

// Returns a feeler that extends to the left of the vehicle.
fn feeler_left() -> Segment {
    Segment::new(Vec2D::new(0f64, 2f64), Vec2D::new(5f64, 7f64)).unwrap()
}

// Returns a feeler that extends to the right of the vehicle.
fn feeler_right() -> Segment {
    Segment::new(Vec2D::new(0f64, -2f64), Vec2D::new(5f64, -7f64)).unwrap()
}

// Returns the feelers corresponding to an arrangement.
fn feelers(arrangement: FeelerShape) -> Vec<Segment> {
    let mut feelers = vec!();
    match arrangement {
        FeelerShape::Spear => feelers.push(feeler_centre()),
        FeelerShape::Fork => {
            feelers.push(feeler_left());
            feelers.push(feeler_right());
        },
        FeelerShape::Trident => {
            feelers.push(feeler_left());
            feelers.push(feeler_centre());
            feelers.push(feeler_right());
        }
    };
    feelers
}

// Returns a wall segment that is near the given feeler.
fn wall_near_feeler(feeler: &Segment, offset: f64) -> Segment {
    // Determine feeler transform.
    let position = feeler.point1;
    let orientation = feeler.unit.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    // Create intersecting segment.
    let intersection = Vec2D::unitx().mul(feeler.length * offset);
    let angle = (0.25f64 + 0.5f64 * random_unity()) * PI;
    let local_point1 = intersection.add(Vec2D::polar(angle, 0.5f64));
    let local_point2 = intersection.add(Vec2D::polar(angle + PI, 0.5f64));

    let point1 = to_world.transform(local_point1);
    let point2 = to_world.transform(local_point2);
    Segment::new(point1, point2).unwrap()
}

// Constructs a scenario for a given arragement of feelers.
fn scenario(shape: FeelerShape, offset: f64) -> Scenario {
    // Work-around for lexical-based borrowing.
    fn walls(feelers: &Vec<Segment>, offset: f64) -> Vec<Segment> {
        let f = |feeler| wall_near_feeler(feeler, offset);
        feelers.iter().map(f).collect()
    }

    // Create vehicle and walls.
    let feelers = feelers(shape);
    let frame = random_frame();
    let to_world = frame.to_world.clone();
    let f = |x: &Segment| x.transform(&to_world);
    let walls = walls(&feelers, offset).iter().map(f).collect();
    Scenario::new(Vehicle::new(frame, feelers), walls)
}

// Constructs scenarios in which feelers and walls do not intersect.
pub fn case1_scenario(shape: FeelerShape) -> Scenario {
    scenario(shape, 1f64 + random_unity())
}

// Constructs scenarios in which one wall intersects with each feeler.
pub fn case2_scenario(shape: FeelerShape) -> Scenario {
    scenario(shape, random_unity())
}
