#![cfg(test)]

use super::scenarios::*;
use super::linalg::vector2d::*;

// Number of each test to execute.
const NUM_RUNS: u32 = 1_000;

// Convenience function for verifying that the expected number of feeler-wall
// intersections occurs.
fn test_scenarios<F, G>( num_scenarios: u32
                       , scenario_creator: F
                       , result_expected: G ) -> bool
    where F: Fn() -> Scenario, G: Fn(Option<Vec2D>) -> bool
{
    let mut success = true;
    for _ in 0..num_scenarios {
        let scenario = scenario_creator();
        let vehicles = scenario.other_vehicles;
        let result = scenario.vehicle.vehicle_avoidance(&vehicles);
        success = success && result_expected(result);
    }
    success
}

// Tests whether vehicle correctly identifies that no steering is required.
#[test]
fn test_case1() {
    let creator = || case1_scenario(3u32);
    let checker = |x: Option<_>| x.is_none();
    assert!(test_scenarios(NUM_RUNS, creator, checker));
}

// Tests whether vehicle correctly identifies that steering is required.
#[test]
fn test_case2() {
    let creator = || case2_scenario(3u32);
    let checker = |x: Option<_>| x.is_some();
    assert!(test_scenarios(NUM_RUNS, creator, checker));
}
