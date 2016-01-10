#![cfg(test)]

use super::common::test_utilities::*;
use super::scenarios::*;

// Tests that Case 1 scenarios produce no potential.
#[test]
fn test_case1() {
    assert!(expect_no_potential(|| case1_scenario(3u32)))
}

// Tests that Case 2 scenarios produce some potential.
#[test]
fn test_case2() {
    assert!(expect_some_potential(|| case2_scenario(3u32)))
}
