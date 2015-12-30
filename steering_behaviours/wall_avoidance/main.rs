extern crate wall_avoidance;
use wall_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;
use utilities::constants::*;

// Invokes test handler using the specified scenario type.
fn time_case<F>(label: String, scenario: F, shape: FeelerShape)
     -> LabelledBatch where F: Fn(FeelerShape) -> Scenario
{
    let creator = || -> Box<HasScenario> { Box::new(scenario(shape)) };
    LabelledBatch::new(label, time_batch(creator, NUM_RUNS, NUM_BATCHES))
}

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = vec!();
        let label11 = format!("Insignificant 1-0 Significant");
        results.push(time_case(label11, case1_scenario, FeelerShape::Spear));

        let label12 = format!("Insignificant 4-0 Significant");
        results.push(time_case(label12, case1_scenario, FeelerShape::Fork));

        let label13 = format!("Insignificant 9-0 Significant");
        results.push(time_case(label13, case1_scenario, FeelerShape::Trident));

        let label21 = format!("Insignificant 0-1 Significant");
        results.push(time_case(label21, case1_scenario, FeelerShape::Spear));

        let label22 = format!("Insignificant 2-2 Significant");
        results.push(time_case(label22, case1_scenario, FeelerShape::Fork));

        let label23 = format!("Insignificant 6-3 Significant");
        results.push(time_case(label23, case1_scenario, FeelerShape::Trident));
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
