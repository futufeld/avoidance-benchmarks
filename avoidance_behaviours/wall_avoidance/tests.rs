#![cfg(test)]

use super::scenarios::scenario_with_obstacles;
use super::utilities::types::{HasScenario, Obstacles};
use super::utilities::test_utilities::test_scenarios;

#[test]
fn test() {
    let creator = |o: &Obstacles| -> Option<Box<HasScenario>> {
        scenario_with_obstacles(o, false)
    };

    for i in 1..6 {
        let obstacles1 = Obstacles::new(i, 0u32);
        assert!(test_scenarios(&obstacles1, |x| creator(x)));
        let obstacles2 = Obstacles::new(0u32, i);
        assert!(test_scenarios(&obstacles2, |x| creator(x)));
    }
}
