use types::*;

use super::common::types::Frame;
use super::linalg::matrix2d::Mat2D;
use super::linalg::vector2d::Vec2D;
use super::utilities::rng_utilities::random_unity;
use super::utilities::types::{HasScenario, Obstacles};

use std::f64::consts::PI;

// Defines feeler arrangements.
#[derive(Copy, Clone)]
pub enum FeelerShape { Spear, Fork, Trident }

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

// Returns the feelers corresponding to an arrangement.
fn feelers(arrangement: FeelerShape) -> Vec<Segment> {
    // Define feelers.
    let feeler_c = Segment::new( Vec2D::new(2f64, 0f64)
                               , Vec2D::new(12f64, 0f64) ).unwrap();
    let feeler_l = Segment::new( Vec2D::new(0f64, 2f64)
                               , Vec2D::new(5f64, 7f64) ).unwrap();
    let feeler_r = Segment::new( Vec2D::new(0f64, -2f64)
                               , Vec2D::new(5f64, -7f64) ).unwrap();

    // Determine feelers to return based on feeler shape.
    let mut feelers = vec!();
    match arrangement {
        FeelerShape::Spear => feelers.push(feeler_c),
        FeelerShape::Fork => {
            feelers.push(feeler_l);
            feelers.push(feeler_r);
        },
        FeelerShape::Trident => {
            feelers.push(feeler_l);
            feelers.push(feeler_c);
            feelers.push(feeler_r);
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
fn scenario(shape: FeelerShape, offset: f64) -> Box<Scenario> {
    // Work-around for lexical borrowing constraint.
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
    Box::new(Scenario::new(Vehicle::new(frame, feelers), walls))
}

// Returns a scenario with the given configuration of obstacles. Returns none
// if it is not possible to create the given scenario.
pub fn scenario_with_obstacles(obstacles: &Obstacles)
    -> Option<Box<HasScenario>>
{
    let offset1 = || random_unity();
    let offset2 = || 1f64 + random_unity();
    match obstacles.details() {
        (1u32, 0u32) => Some(scenario(FeelerShape::Spear, offset2())),
        (4u32, 0u32) => Some(scenario(FeelerShape::Fork, offset2())),
        (9u32, 0u32) => Some(scenario(FeelerShape::Trident, offset2())),
        (0u32, 1u32) => Some(scenario(FeelerShape::Spear, offset1())),
        (2u32, 2u32) => Some(scenario(FeelerShape::Fork, offset1())),
        (6u32, 3u32) => Some(scenario(FeelerShape::Trident, offset1())),
        _ => None
    }
}
