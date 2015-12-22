use types::*;

use super::linalg::vector2d::*;
use super::linalg::matrix2d::*;
use super::utilities::handler::*;
use super::common::vehicle::Vehicle;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

use std::f64::consts::PI;

//
pub struct Scenario {
    pub vehicle: FeelerVehicle,
    pub walls: Vec<Segment>
}

impl HasScenario for Scenario {
    //
    fn run(&mut self) {
        self.vehicle.update();
        let _ = self.vehicle.wall_avoidance(&self.walls);
    }
}

impl Scenario {
    //
    pub fn new(vehicle: FeelerVehicle, walls: Vec<Segment>) -> Scenario {
        Scenario { vehicle: vehicle, walls: walls }
    }
}

//
#[derive(Copy, Clone)]
pub enum FeelerShape { Spear, Fork, Trident }

// Returns a random f64 between 0 and 1 using the thread's random number
// generator.
fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

//
fn random_vehicle() -> Vehicle {
    let angle = 2f64 * PI * random_unity();
    let position = Vec2D::polar(angle, 100f64 * random_unity());
    Vehicle::new(position, 2f64 * PI * random_unity())
}

//
fn feeler_centre() -> Segment {
    Segment::new(Vec2D::new(2f64, 0f64), Vec2D::new(12f64, 0f64)).unwrap()
}

//
fn feeler_left() -> Segment {
    Segment::new(Vec2D::new(0f64, 2f64), Vec2D::new(5f64, 7f64)).unwrap()
}

//
fn feeler_right() -> Segment {
    Segment::new(Vec2D::new(0f64, -2f64), Vec2D::new(5f64, -7f64)).unwrap()
}

//
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

//
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

//
fn scenario(shape: FeelerShape, offset: f64) -> Scenario {
    // Create feelers and walls.
    let mut local_walls = vec!();
    let feelers = feelers(shape);
    for feeler in feelers.iter() {
        local_walls.push(wall_near_feeler(feeler, offset))
    }

    // Transform walls into world space from vehicle's local space.
    let vehicle = random_vehicle();
    let to_world = vehicle.to_world.clone();
    let f = |x: &Segment| x.transform(&to_world);
    let walls = local_walls.iter().map(f).collect();
    let feeler_vehicle = FeelerVehicle::new(vehicle, feelers);
    Scenario::new(feeler_vehicle, walls)
}

//
pub fn case1_scenario(shape: FeelerShape) -> Scenario {
    scenario(shape, 1f64 + random_unity())
}

//
pub fn case2_scenario(shape: FeelerShape) -> Scenario {
    scenario(shape, random_unity())
}
