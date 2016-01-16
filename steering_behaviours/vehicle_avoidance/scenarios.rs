use types::*;

use super::common::test_utilities::TestableScenario;
use super::linalg::vector2d::*;
use super::utilities::handler::*;
use super::utilities::utilities::random_unity;

use std::f64::consts::PI;

// Radius of vehicles in scenarios.
const VEHICLE_RADIUS: f64 = 5f64;

// Radius of virtual circle used to position scenario.
const MAX_ACCELERATION: f64 = 25f64;

// Arrangement of vehicles.
pub struct Scenario { pub vehicle: Vehicle
                    , pub other_vehicles: Vec<Vehicle> }

impl HasScenario for Scenario {
    // Runs the scenario.
    fn run(&mut self) {
        let _ = self.vehicle.vehicle_avoidance(&self.other_vehicles);
    }
}

impl TestableScenario for Scenario {
    // Returns the interactions between the vehicle and obstacles in the
    // scenario.
    fn interactions(&self) -> u32 {
        let mut count = 0;
        for vehicle in self.other_vehicles.iter() {
            if self.vehicle.interaction(vehicle).is_some() { count += 1; }
        }
        count
    }

    // Returns the avoidance force to be applied to the vehicle according to
    // the steering scenario.
    fn avoidance(&self) -> Option<Vec2D> {
        self.vehicle.vehicle_avoidance(&self.other_vehicles)
    }
}

impl Scenario {
    // Creates a scenario involving multiple vehicles.
    pub fn new(vehicle: Vehicle, other_vehicles: Vec<Vehicle>) -> Scenario {
        Scenario { vehicle: vehicle, other_vehicles: other_vehicles }
    }
}

// Returns a scenario involving vehicles that either will or will not collide
// at their current velocities.
fn scenario(num_vehicles: u32, colliding: bool) -> Box<Scenario> {
    // Create semi-random vehicle.
    let focus_position = Vec2D::polar(2f64 * PI * random_unity(), 100f64);
    let focus_angle = 2f64 * PI * random_unity();
    let focus_speed = 10f64;
    let focus_velocity = Vec2D::polar(focus_angle, focus_speed);

    let focus_vehicle = Vehicle::new( focus_position
                                    , focus_velocity
                                    , VEHICLE_RADIUS
                                    , MAX_ACCELERATION );

    // Create vehicles in proximity of focus vehicle.
    let mut other_vehicles = vec!();
    for _ in 0..num_vehicles {
        let time = 1f64 + 4f64 * random_unity();
        let offset = focus_velocity.mul(time);
        let intersection = focus_position.add(offset);

        let angle_offset = PI * (0.05f64 + 0.4f64 * random_unity());
        let other_angle = focus_angle + angle_offset;
        let other_speed = 10f64;
        let mut other_velocity = Vec2D::polar(other_angle, other_speed);
        if random_unity() < 0.5 { other_velocity = other_velocity.neg(); }

        let other_travel = other_velocity.neg().mul(time);
        if !colliding { other_velocity = other_velocity.neg(); }
        let other_position = intersection.add(other_travel);

        let other_vehicle = Vehicle::new( other_position
                                        , other_velocity
                                        , VEHICLE_RADIUS
                                        , MAX_ACCELERATION );
        other_vehicles.push(other_vehicle);
    }
    Box::new(Scenario::new(focus_vehicle, other_vehicles))
}

// Returns a scenario with the given configuration of obstacles. Returns none
// if it is not possible to create the given scenario.
pub fn scenario_with_obstacles(obstacles: &Obstacles)
    -> Option<Box<HasScenario>>
{
    match obstacles.details() {
        (num_obs, 0u32) => Some(scenario(num_obs, false)),
        (0u32, num_obs) => Some(scenario(num_obs, true)),
        _ => None
    }
}
