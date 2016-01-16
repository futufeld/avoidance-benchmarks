extern crate vehicle_avoidance;
use vehicle_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = vec!();
        for i in 1..6 {
            let creator1 = || -> Box<HasScenario> {
                Box::new(case1_scenario(i))
            };
            let interaction1 = Obstacles::none_significant(i);
            let batch1 = time_batch(creator1, NUM_RUNS);
            let results1 = ObstacleBatch::new(interaction1, batch1);
            results.push(results1);

            let creator2 = || -> Box<HasScenario> {
                Box::new(case2_scenario(i))
            };
            let interaction2 = Obstacles::all_significant(i);
            let batch2 = time_batch(creator2, NUM_RUNS);
            let results2 = ObstacleBatch::new(interaction2, batch2);
            results.push(results2);
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
