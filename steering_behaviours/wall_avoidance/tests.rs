#![cfg(test)]

use super::common::test_utilities::*;
use super::scenarios::*;

// Tests whether vehicle's feeler correctly identifies case 1 scenarios.
#[test]
fn test_case1() {
    let creator = || Box::new(case1_scenario(FeelerShape::Spear));
    assert!(expected_interactions(|| creator(), 0u32));
    assert!(expected_avoidance(|| creator(), false));

    let creator = || Box::new(case1_scenario(FeelerShape::Fork));
    assert!(expected_interactions(|| creator(), 0u32));
    assert!(expected_avoidance(|| creator(), false));

    let creator = || Box::new(case1_scenario(FeelerShape::Trident));
    assert!(expected_interactions(|| creator(), 0u32));
    assert!(expected_avoidance(|| creator(), false));
}

// Tests whether vehicle's feeler correctly identifies case 2 scenarios.
#[test]
fn test_case2() {
    let creator = || Box::new(case2_scenario(FeelerShape::Spear));
    assert!(expected_interactions(|| creator(), 1u32));
    assert!(expected_avoidance(|| creator(), true));

    let creator = || Box::new(case2_scenario(FeelerShape::Fork));
    assert!(expected_interactions(|| creator(), 2u32));
    assert!(expected_avoidance(|| creator(), true));

    let creator = || Box::new(case2_scenario(FeelerShape::Trident));
    assert!(expected_interactions(|| creator(), 3u32));
    assert!(expected_avoidance(|| creator(), true));
}
