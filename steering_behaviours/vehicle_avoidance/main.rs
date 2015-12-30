extern crate vehicle_avoidance;
use vehicle_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;
use utilities::constants::*;

// Invokes test handler using the specified scenario type.
fn time_case<F>(label: String, scenario: F, num_obstacles: u32)
    -> LabelledBatch where F: Fn(u32) -> Scenario
{
    let creator = || -> Box<HasScenario> { Box::new(scenario(num_obstacles)) };
    LabelledBatch::new(label, time_batch(creator, NUM_RUNS, NUM_BATCHES))
}

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = vec!();
        for n in 1..6 {
            let label1 = format!("Insignificant {}-0 Significant", n);
            results.push(time_case(label1, case1_scenario, n));
            let label2 = format!("Insignificant 0-{} Significant", n);
            results.push(time_case(label2, case2_scenario, n));
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
