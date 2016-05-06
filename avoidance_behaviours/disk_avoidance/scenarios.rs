use types::*;

use super::common::types::Frame;
use super::linalg::matrix2d::Mat2D;
use super::linalg::vector2d::Vec2D;
use super::utilities::rng_utilities::*;
use super::utilities::types::{HasScenario, Obstacles};

use std::f64::consts::PI;

// Arrangement of vehicle and disks to be used in benchmarks.
pub struct Scenario { pub vehicle: Vehicle
                    , pub disks:   Vec<Disk> }

impl HasScenario for Scenario {
    // Returns the interactions between the vehicle and obstacles in the
    // scenario.
    fn interactions(&self) -> u32 {
        let mut count = 0;
        for vehicle in self.disks.iter() {
            if self.vehicle.interaction(vehicle).is_some() { count += 1; }
        }
        count
    }

    // Returns the avoidance force to be applied to the vehicle according to
    // the steering scenario.
    fn avoidance(&mut self) -> Option<Vec2D> {
        self.vehicle.disk_avoidance(&self.disks)
    }
}

impl Scenario {
    // Convenience function for creating scenarios.
    fn new(vehicle: Vehicle, disks: Vec<Disk>) -> Scenario {
        Scenario { vehicle: vehicle, disks: disks }
    }
}

// Returns a vehicle with a semi-random position and orientation with the
// given length and width.
fn random_vehicle(length: f64, width: f64) -> Vehicle {
    let position = Vec2D::polar(random_tau(), 100f64 * random_unity());
    let orientation = 2f64 * PI * random_unity();
    let vehicle = Frame::new(position, orientation);
    Vehicle::new(vehicle, length, width)
}

// Returns a disk with a semi-random centre determined by `x_scale`, `y_scale`
// and `y_offset`, which is then transformed by `transform`.
fn near_disk(x_scale: f64, y_scale: f64, significant: bool, transform: &Mat2D)
    -> Disk
{
    let radius = y_scale * random_margin();
    let mut offset = random_margin();
    if !significant { offset += 1f64; }

    let local_x = x_scale * random_unity();
    let mut local_y = radius + y_scale * offset;
    if random_unity() < 0.5f64 { local_y = -local_y; }

    let local_centre = Vec2D::new(local_x, local_y);
    let centre = transform.transform(local_centre);
    Disk::new(centre, radius)
}

// Returns a semi-random scenario involving `n` obstacles positioned with
// respect to `x_scale`, `y_scale` and `y_offset` (see `near_disk`).
fn scenario(n: u32, x_scale: f64, y_scale: f64, significant: bool)
    -> Box<Scenario>
{
    let vehicle = random_vehicle(x_scale, y_scale);
    let to_world = vehicle.frame.to_world.clone();
    let f = |_| near_disk(x_scale, y_scale, significant, &to_world);
    let disks: Vec<Disk> = (0..n).map(f).collect();
    Box::new(Scenario::new(vehicle, disks))
}

// Returns a scenario with the given configuration of obstacles. Returns none
// if it is not possible to create the given scenario.
pub fn scenario_with_obstacles( obstacles:     &Obstacles
                              , feeler_length: f64
                              , feeler_width:  f64 )
    -> Option<Box<HasScenario>>
{
    match obstacles.details() {
        (num_obs, 0u32) =>
            Some(scenario(num_obs, feeler_length, feeler_width, false)),
        (0u32, num_obs) =>
            Some(scenario(num_obs, feeler_length, feeler_width, true)),
        _ => None
    }
}
