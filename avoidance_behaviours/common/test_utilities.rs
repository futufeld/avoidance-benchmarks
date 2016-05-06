use super::utilities::types::HasScenario;

// Number of each test to execute.
const NUM_RUNS: u32 = 10_000;

// Verifies that the expected number of interactions occur.
pub fn expected_interactions<F>(creator: F, expected: u32) -> bool
    where F: Fn() -> Box<HasScenario>
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
    where F: Fn() -> Box<HasScenario>
{
    for _ in 0..NUM_RUNS {
        match creator().avoidance() {
            Some(_) => if !expected { return false },
            None => if expected { return false }
        }
    }
    true
}
