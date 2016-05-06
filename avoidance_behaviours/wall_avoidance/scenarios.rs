use types::*;

use super::common::types::Frame;
use super::linalg::matrix2d::Mat2D;
use super::linalg::vector2d::Vec2D;
use super::utilities::rng_utilities::{random_margin, random_unity};
use super::utilities::types::{HasScenario, Obstacles};

use std::f64::consts::PI;

// Arrangement of vehicle and line segment obstacles.
pub struct Scenario { pub vehicle: Vehicle
                    , pub walls: Vec<Segment> }

impl HasScenario for Scenario {
    // Returns the interactions between the vehicle and obstacles in the
    // scenario.
    fn interactions(&self) -> u32 {
        let mut count = 0;
        for feeler in self.vehicle.local_feelers.iter() {
            for wall in self.walls.iter() {
                if self.vehicle.interaction(&feeler, wall).is_some() {
                    count += 1;
                }
            }
        }
        count
    }

    // Returns the avoidance force to be applied to the vehicle according to
    // the steering scenario.
    fn avoidance(&mut self) -> Option<Vec2D> {
        self.vehicle.update();
        self.vehicle.wall_avoidance(&self.walls)
    }
}

impl Scenario {
    // Creates a scenario involving a vehicle with feelers and wall segments.
    pub fn new(vehicle: Vehicle, walls: Vec<Segment>) -> Scenario {
        Scenario { vehicle: vehicle, walls: walls }
    }
}

// Returns a frame with semi-random position and orientation.
fn random_frame() -> Frame {
    let angle = 2f64 * PI * random_unity();
    let position = Vec2D::polar(angle, 100f64 * random_unity());
    Frame::new(position, 2f64 * PI * random_unity())
}

// Returns a feeler that extends ahead of a vehicle.
fn feeler() -> Segment {
    Segment::new( Vec2D::new(2f64, 0f64), Vec2D::new(12f64, 0f64) ).unwrap()
}

// Returns a vector of whiskers - feelers that extend to the left and right of
// a hypothetical vehicle.
fn whiskers() -> Vec<Segment> {
    let l = Segment::new( Vec2D::new(0f64,  2f64), Vec2D::new(5f64,  7f64) );
    let r = Segment::new( Vec2D::new(0f64, -2f64), Vec2D::new(5f64, -7f64) );
    vec!(l.unwrap(), r.unwrap())
}

// Returns a wall segment that is near the given feeler.
fn wall_near_feeler(feeler: &Segment, significant: bool) -> Segment {
    // Determine feeler transform.
    let position = feeler.point1;
    let orientation = feeler.unit.angle();
    let to_world = Mat2D::rotation(orientation).shift(position);

    // Create intersecting segment.
    let mut offset = random_margin();
    if !significant { offset += 1f64; }
    let intersection = Vec2D::unitx().mul(feeler.length * offset);
    let angle = (0.25f64 + 0.5f64 * random_unity()) * PI;
    let local_point1 = intersection.add(Vec2D::polar(angle, 0.5f64));
    let local_point2 = intersection.add(Vec2D::polar(angle + PI, 0.5f64));

    let point1 = to_world.transform(local_point1);
    let point2 = to_world.transform(local_point2);
    Segment::new(point1, point2).unwrap()
}

// Constructs a scenario for a given arragement of feelers.
fn scenario(num_obstacles: u32, significant: bool, has_whiskers: bool)
    -> Box<Scenario>
{
    let feeler = feeler();
    let frame = random_frame();
    let to_world = frame.to_world.clone();

    let f = |_| wall_near_feeler(&feeler, significant).transform(&to_world);
    let walls = (0..num_obstacles).map(f).collect();

    let mut feelers = vec!(feeler.clone());
    if has_whiskers { feelers.extend(whiskers()); }
    Box::new(Scenario::new(Vehicle::new(frame, feelers), walls))
}

// Returns a scenario with the given configuration of obstacles. Returns none
// if it is not possible to create the given scenario.
pub fn scenario_with_obstacles(obstacles: &Obstacles, has_whiskers: bool)
    -> Option<Box<HasScenario>>
{
    match obstacles.details() {
        (num_obs, 0u32) => Some(scenario(num_obs, false, has_whiskers)),
        (0u32, num_obs) => Some(scenario(num_obs, true, has_whiskers)),
        _ => None
    }
}
