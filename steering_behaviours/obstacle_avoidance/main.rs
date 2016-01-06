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

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = vec!();
        for i in 1..6 {
            let creator1 = || -> Box<HasScenario> {
                Box::new(case1_scenario(i, FEELER_LENGTH, FEELER_WIDTH))
            };
            let interaction1 = Obstacles::none_significant(i);
            let batch1 = time_batch(creator1, NUM_RUNS, NUM_BATCHES);
            let results1 = ObstacleBatch::new(interaction1, batch1);
            results.push(results1);

            let creator2 = || -> Box<HasScenario> {
                Box::new(case2_scenario(i, FEELER_LENGTH, FEELER_WIDTH))
            };
            let interaction2 = Obstacles::all_significant(i);
            let batch2 = time_batch(creator2, NUM_RUNS, NUM_BATCHES);
            let results2 = ObstacleBatch::new(interaction2, batch2);
            results.push(results2);
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
