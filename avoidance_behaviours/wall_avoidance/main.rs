extern crate wall_avoidance;
use wall_avoidance::scenarios::*;

extern crate utilities;
use utilities::bench_utilities::time_batch;
use utilities::types::{HasScenario, Obstacles};
use utilities::utilities::{time_execution_seconds, write_results};

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let creator = |o: &Obstacles| -> Box<HasScenario> {
            scenario_with_obstacles(o, false).unwrap()
        };

        let mut results = vec!();
        for i in 1..6 {
            let obstacles1 = Obstacles::new(i, 0u32);
            results.push(time_batch(&obstacles1, |x| creator(x)));
            let obstacles2 = Obstacles::new(0u32, i);
            results.push(time_batch(&obstacles2, |x| creator(x)));
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
