use super::linalg::vector2d::Vec2D;

// Number of each test to execute.
const NUM_RUNS: u32 = 10_000;

// For scenarios that are testable without needing to access internals.
pub trait TestableScenario {
    fn interactions(&self) -> u32;
    fn avoidance(&self) -> Option<Vec2D>;
}

// Verifies that the expected number of interactions occur.
pub fn expected_interactions<F>(creator: F, expected: u32) -> bool
    where F: Fn() -> Box<TestableScenario>
{
    for _ in 0..NUM_RUNS {
        if creator().interactions() != expected {
            return false;
        }
    }
    true
}

// Verifies whether a avoidance force was produced.
pub fn expected_avoidance<F>(creator: F, expected: bool) -> bool
    where F: Fn() -> Box<TestableScenario>
{
    for _ in 0..NUM_RUNS {
        match creator().avoidance() {
            Some(_) => if !expected { return false },
            None => if expected { return false }
        }
    }
    true
}
