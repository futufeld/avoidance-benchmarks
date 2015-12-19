extern crate obstacle_avoidance;
use obstacle_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;

use std::path::Path;

// Number of scenarios to run for each benchmark.
const NUM_RUNS: u32 = 1_000_000;

// Number of batches of runs for each benchmark.
const NUM_BATCHES: u32 = 100;

// Length of the feeler.
const FEELER_LENGTH: f64 = 10f64;

// Width of the feeler.
const FEELER_WIDTH: f64 = 2f64;

// Used to indicate scenario type.
enum ScenarioType { Case1, Case2, Case3 }

// Returns the string defined by the given scenario type.
fn label(case: ScenarioType) -> String {
    match case {
        ScenarioType::Case1 => format!("case1"),
        ScenarioType::Case2 => format!("case2"),
        ScenarioType::Case3 => format!("case3")
    }
}

// Invokes test handler using the specified scenario type.
fn time_case(case: ScenarioType) -> LabelledBatch {
    let scenario: fn(f64, f64) -> Scenario = match case {
        ScenarioType::Case1 => case1_scenario,
        ScenarioType::Case2 => case2_scenario,
        ScenarioType::Case3 => case3_scenario
    };
    let creator = || -> Box<HasScenario> {
        Box::new(scenario(FEELER_LENGTH, FEELER_WIDTH))
    };
    let batch = time_batch(creator, NUM_RUNS, NUM_BATCHES);
    LabelledBatch::new(label(case), batch)
}

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = vec!();
        results.push(time_case(ScenarioType::Case1));
        results.push(time_case(ScenarioType::Case2));
        results.push(time_case(ScenarioType::Case3));

        match get_filepath() {
            Some(filestring) => {
                let filepath = Path::new(&filestring);
                write_batches(&filepath, results);
            },
            None => ()
        }
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
