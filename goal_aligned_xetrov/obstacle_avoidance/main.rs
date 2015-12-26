extern crate obstacle_avoidance;
use obstacle_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;
use utilities::constants::*;

// Used to indicate scenario type.
enum ScenarioType { Case1, Case2 }

// Returns the string defined by the given scenario type and number of
// obstacles.
fn label(case: ScenarioType, num_obstacles: u32) -> String {
    match case {
        ScenarioType::Case1 => format!("case1-{}", num_obstacles),
        ScenarioType::Case2 => format!("case2-{}", num_obstacles)
    }
}

// Invokes test handler using the specified scenario type and number of
// obstacles.
fn time_case(case: ScenarioType, num_obstacles: u32) -> LabelledBatch {
    let scenario: fn(u32) -> Scenario = match case {
        ScenarioType::Case1 => case1_scenario,
        ScenarioType::Case2 => case2_scenario
    };
    let creator = || -> Box<HasScenario> { Box::new(scenario(num_obstacles)) };
    let batch = time_batch(creator, NUM_RUNS, NUM_BATCHES);
    LabelledBatch::new(label(case, num_obstacles), batch)
}

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = vec!();
        for i in 1..6 {
            results.push(time_case(ScenarioType::Case1, i));
            results.push(time_case(ScenarioType::Case2, i));
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
