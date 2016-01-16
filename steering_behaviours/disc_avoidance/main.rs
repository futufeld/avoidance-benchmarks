extern crate obstacle_avoidance;
use obstacle_avoidance::scenarios::*;

extern crate utilities;
use utilities::bench_utilities::time_batch;
use utilities::types::{HasScenario, Obstacles};
use utilities::utilities::{time_execution_seconds, write_results};

// Length of the feeler.
const FEELER_LENGTH: f64 = 10f64;

// Width of the feeler.
const FEELER_WIDTH: f64 = 2f64;

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let creator = |o: &Obstacles| -> Box<HasScenario> {
            scenario_with_obstacles(o, FEELER_LENGTH, FEELER_WIDTH).unwrap()
        };

        let mut results = vec!();
        for i in 1..6 {
            let interaction1 = Obstacles::new(i, 0u32);
            results.push(time_batch(&interaction1, |x| creator(x)));
            let interaction2 = Obstacles::new(0u32, i);
            results.push(time_batch(&interaction2, |x| creator(x)));
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
