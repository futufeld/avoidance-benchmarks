use super::types::*;

use super::linalg::vector2d::*;

// Number of each test to execute.
pub const NUM_RUNS: u32 = 1_000;

// Convenience function for verifying the existence of certain cases in
// generated scenarios.
pub fn test_scenarios<F, G>( num_scenarios: u32
                           , scenario_creator: F
                           , result_expected: G) -> bool
    where F: Fn() -> Scenario, G: Fn(Option<Vec2D>) -> bool
{
    let mut success = true;
    for _ in 0..num_scenarios {
        let scenario = scenario_creator();
        let result = scenario.vehicle.total_potential(&scenario.obstacles);
        success = success && result_expected(result);
    }
    success
}

// Tests whether scenarios yield no potential when all obstacles are distant
// from the scenario vehicle.
pub fn expect_no_potential<F>(creator: F) -> bool
    where F: Fn() -> Scenario
{
    let checker = |x: Option<Vec2D>| { x.is_none() };
    test_scenarios(NUM_RUNS, creator, checker)
}

// Tests whether scenarios yield any potential when all obstacles are near to
// the scenario vehicle.
pub fn expect_some_potential<F>(creator: F) -> bool
    where F: Fn() -> Scenario
{
    let checker = |x: Option<Vec2D>| x.is_some();
    test_scenarios(NUM_RUNS, creator, checker)
}
