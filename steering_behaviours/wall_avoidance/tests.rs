#![cfg(test)]

use super::scenarios::scenario_with_obstacles;
use super::utilities::types::Obstacles;
use super::utilities::test_utilities::test_scenarios;

#[test]
fn test() {
    let mut obstacles = vec!();
    obstacles.push(Obstacles::new(1u32, 0u32));
    obstacles.push(Obstacles::new(4u32, 0u32));
    obstacles.push(Obstacles::new(9u32, 0u32));
    obstacles.push(Obstacles::new(0u32, 1u32));
    obstacles.push(Obstacles::new(2u32, 2u32));
    obstacles.push(Obstacles::new(6u32, 3u32));

    for obstacle in obstacles.iter() {
        assert!(test_scenarios(&obstacle, scenario_with_obstacles));
    }
}
