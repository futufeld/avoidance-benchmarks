#![cfg(test)]

use super::scenarios::*;

use super::common::test_utilities::*;

// Tests whether vehicle's feeler correctly identifies case 1 scenarios.
#[test]
fn test_case1() {
    let creator = || Box::new(case1_scenario(3u32));
    assert!(expected_interactions(|| creator(), 0u32));
    assert!(expected_avoidance(|| creator(), false));
}

// Tests whether vehicle's feeler correctly identifies case 2 scenarios.
#[test]
fn test_case2() {
    let creator = || Box::new(case2_scenario(3u32));
    assert!(expected_interactions(|| creator(), 3u32));
    assert!(expected_avoidance(|| creator(), true));
}
