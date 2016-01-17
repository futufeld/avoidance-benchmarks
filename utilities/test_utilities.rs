use super::types::{HasScenario, Obstacles};

// Number of each test to execute.
pub const NUM_RUNS: u32 = 1_000;

// Tests whether the scenario produces the expected number of interactions and
// avoidance force.
pub fn test_scenarios<F>(obstacles: &Obstacles, creator: F) -> bool
    where F: Fn(&Obstacles) -> Option<Box<HasScenario>>
{
    let significance = obstacles.significant > 0;
    for _ in 0..NUM_RUNS {
        let mut scenario = match creator(obstacles) {
            Some(scenario) => scenario,
            None => {
                println!("Creation function failed to return scenario");
                return false
            }
        };

        // Check whether the expected number of interactions occurred.
        if scenario.interactions() != obstacles.significant {
            println!("Unexpected number of interactions");
            return false;
        }

        // Check whether an avoidance force was generated.
        let avoidance = scenario.avoidance().is_some();
        if (avoidance && !significance) || (!avoidance && significance) {
            println!("Unexpected avoidance result");
            return false;
        }
    }
    return true;
}
