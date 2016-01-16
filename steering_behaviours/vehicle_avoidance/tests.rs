#![cfg(test)]

use super::scenarios::scenario_with_obstacles;
use super::utilities::test_utilities::test_scenarios;
use super::utilities::types::Obstacles;

#[test]
fn test() {
    for i in 1..6 {
        let obstacles1 = Obstacles::new(i, 0u32);
        assert!(test_scenarios(&obstacles1, scenario_with_obstacles));
        let obstacles2 = Obstacles::new(0u32, i);
        assert!(test_scenarios(&obstacles2, scenario_with_obstacles));
    }
}
