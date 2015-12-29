extern crate obstacle_avoidance;
use obstacle_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;
use utilities::constants::*;

// Length of the feeler.
const FEELER_LENGTH: f64 = 10f64;

// Width of the feeler.
const FEELER_WIDTH: f64 = 2f64;

// Used to indicate scenario type.
enum ScenarioType { Case1, Case2 }

// Returns the string defined by the given scenario type.
fn label(case: ScenarioType, num_obstacles: u32) -> String {
    let (insignificant, significant) = match case {
        ScenarioType::Case1 => (num_obstacles, 0u32),
        ScenarioType::Case2 => (0u32, num_obstacles)
    };
    format!("Insignificant {}-{} Significant", insignificant, significant)
}

// Invokes test handler using the specified scenario type.
fn time_case(case: ScenarioType, num_obstacles: u32) -> LabelledBatch {
    let scenario: fn(u32, f64, f64) -> Scenario = match case {
        ScenarioType::Case1 => case1_scenario,
        ScenarioType::Case2 => case2_scenario
    };
    let creator = || -> Box<HasScenario> {
        Box::new(scenario(num_obstacles, FEELER_LENGTH, FEELER_WIDTH))
    };
    let batch = time_batch(creator, NUM_RUNS, NUM_BATCHES);
    LabelledBatch::new(label(case, num_obstacles), batch)
}

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = vec!();
        for num_obstacles in 1..6 {
            results.push(time_case(ScenarioType::Case1, num_obstacles));
            results.push(time_case(ScenarioType::Case2, num_obstacles));
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
