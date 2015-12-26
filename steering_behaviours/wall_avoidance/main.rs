extern crate wall_avoidance;
use wall_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;
use utilities::constants::*;

// Used to indicate scenario type.
enum ScenarioType { Case1, Case2 }

// Returns the string defined by the given scenario type.
fn label(case: ScenarioType, shape: FeelerShape) -> String {
    match case {
        ScenarioType::Case1 => match shape {
            FeelerShape::Spear => format!("0-1"),
            FeelerShape::Fork => format!("2-2"),
            FeelerShape::Trident => format!("3-6")
        },
        ScenarioType::Case2 => match shape {
            FeelerShape::Spear => format!("1-0"),
            FeelerShape::Fork => format!("4-0"),
            FeelerShape::Trident => format!("9-0")
        }
    }
}

// Invokes test handler using the specified scenario type.
fn time_case(case: ScenarioType, shape: FeelerShape) -> LabelledBatch {
    let scenario: fn(FeelerShape) -> Scenario = match case {
        ScenarioType::Case1 => case1_scenario,
        ScenarioType::Case2 => case2_scenario
    };
    let creator = || -> Box<HasScenario> { Box::new(scenario(shape)) };
    let batch = time_batch(creator, NUM_RUNS, NUM_BATCHES);
    LabelledBatch::new(label(case, shape), batch)
}

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = vec!();
        results.push(time_case(ScenarioType::Case1, FeelerShape::Spear));
        results.push(time_case(ScenarioType::Case1, FeelerShape::Fork));
        results.push(time_case(ScenarioType::Case1, FeelerShape::Trident));
        results.push(time_case(ScenarioType::Case2, FeelerShape::Spear));
        results.push(time_case(ScenarioType::Case2, FeelerShape::Fork));
        results.push(time_case(ScenarioType::Case2, FeelerShape::Trident));
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
