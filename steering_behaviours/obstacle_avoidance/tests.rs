#![cfg(test)]

use super::scenarios::*;
use super::types::*;

// Number of each test to execute.
pub const NUM_RUNS: u32 = 1_000;

// Convenience function for verifying that `Feeler` correctly identifies the
// case of `num_scenarios` scenarios.
pub fn test_scenarios<F, G>( num_scenarios: u32
                           , scenario_creator: F
                           , result_expected: G) -> bool
    where F: Fn() -> Scenario, G: Fn(FeelerResult) -> bool
{
    let mut success = true;
    for scenario in create_scenarios(num_scenarios, scenario_creator).iter() {
        for circle in scenario.circles.iter() {
            let result = scenario.feeler.obstacle_intersections(circle);
            success = success && result_expected(result);
        }
    }
    success
}

// Tests whether `Feeler` correctly identifies case 1 scenarios.
#[test]
fn test_case1() {
    let creator = || case1_scenario(3u32, 10f64, 2f64);
    let checker = |to_check: FeelerResult| match to_check {
        FeelerResult::Case1 => true,
        _ => false
    };
    test_scenarios(NUM_RUNS, creator, checker);
}

// Tests whether `Feeler` correctly identifies case 2 scenarios.
#[test]
fn test_case2() {
    let creator = || case1_scenario(3u32, 10f64, 2f64);
    let checker = |to_check: FeelerResult| match to_check {
        FeelerResult::Case2(_) => true,
        _ => false
    };
    test_scenarios(NUM_RUNS, creator, checker);
}

// Tests whether `Feeler` correctly identifies case 3 scenarios.
#[test]
fn test_case3() {
    let creator = || case1_scenario(3u32, 10f64, 2f64);
    let checker = |to_check: FeelerResult| match to_check {
        FeelerResult::Case3(_) => true,
        _ => false
    };
    test_scenarios(NUM_RUNS, creator, checker);
}
