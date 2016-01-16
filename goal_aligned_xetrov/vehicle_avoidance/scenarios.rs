use super::common::types::*;
use super::linalg::vector2d::Vec2D;
use super::utilities::rng_utilities::*;
use super::utilities::types::{HasScenario, Obstacles};

// Returns a semi-random vehicle near to the given position.
fn near_vehicle(position: Vec2D, significant: bool, potential_scale: f64)
    -> Box<HasSource>
{
    let mut offset1 = random_margin();
    if !significant { offset1 += 1f64; }
    let future_position = position.add(
        Vec2D::polar( random_tau(), offset1 * POTENTIAL_SCALE )
    );

    let offset2 = 0.25f64 * potential_scale * random_unity();
    let velocity = Vec2D::polar(random_tau(), offset2);
    let position = future_position.sub(velocity.mul(LOOK_AHEAD));

    Box::new(Vehicle::new(position, velocity, potential_scale))
}

// Returns a semi-random scenario involving a single vehicle inside or outside,
// depending on offset, the loci of influence of a number of other vehicles.
fn scenario(num_obstacles: u32, significant: bool) -> Box<Scenario> {
    let vehicle = random_vehicle();
    let position = vehicle.look_ahead();
    let f = |_| near_vehicle(position, significant, POTENTIAL_SCALE);
    let obstacles = (0..num_obstacles).map(f).collect();
    Box::new(Scenario::new(vehicle, obstacles))
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
