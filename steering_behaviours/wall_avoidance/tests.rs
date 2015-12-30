#![cfg(test)]

use super::scenarios::*;
use super::types::*;

// Number of each test to execute.
const NUM_RUNS: u32 = 1_000;

// Convenience function for verifying that the expected number of feeler-wall
// intersections occurs.
fn test_scenarios<F, G>( num_scenarios: u32
                       , scenario_creator: F
                       , result_expected: G ) -> bool
    where F: Fn() -> Scenario, G: Fn(Vec<Interaction>) -> bool
{
    let mut success = true;
    for _ in 0..num_scenarios {
        let scenario = scenario_creator();
        let result = scenario.vehicle.wall_interactions(&scenario.walls);
        success = success && result_expected(result);
    }
    success
}

// Test all feeler arrangements in scenarios in which no obstacles intersect
// feelers.
#[test]
fn test_case1() {
    let creator = || case1_scenario(FeelerShape::Spear);
    let checker = |x: Vec<_>| x.len() == 0;
    assert!(test_scenarios(NUM_RUNS, creator, checker));

    let creator = || case1_scenario(FeelerShape::Fork);
    let checker = |x: Vec<_>| x.len() == 0;
    assert!(test_scenarios(NUM_RUNS, creator, checker));

    let creator = || case1_scenario(FeelerShape::Trident);
    let checker = |x: Vec<_>| x.len() == 0;
    assert!(test_scenarios(NUM_RUNS, creator, checker));
}

// Test all feeler arrangements in scenarios in which exactly one obstacle
// intersects each feeler.
#[test]
fn test_case2() {
    let creator = || case2_scenario(FeelerShape::Spear);
    let checker = |x: Vec<_>| x.len() == 1;
    assert!(test_scenarios(NUM_RUNS, creator, checker));

    let creator = || case2_scenario(FeelerShape::Fork);
    let checker = |x: Vec<_>| x.len() == 2;
    assert!(test_scenarios(NUM_RUNS, creator, checker));

    let creator = || case2_scenario(FeelerShape::Trident);
    let checker = |x: Vec<_>| x.len() == 3;
    assert!(test_scenarios(NUM_RUNS, creator, checker));
}
