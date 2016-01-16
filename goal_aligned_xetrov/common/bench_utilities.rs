use super::types::Scenario;

extern crate utilities;
use utilities::handler::*;
use utilities::constants::*;
use utilities::utilities::*;

extern crate time;
use self::time::PreciseTime;

// Runs benchmarks and saves results to a file specified on the command line.
// Functions case1_scenario and case2_scenario are expected to produce
// scenarios that involve none and some/all collision risks respectively.
pub fn run_benchmarks<F, G>(case1_scenario: F, case2_scenario: G)
    where F: Fn(u32) -> Scenario, G: Fn(u32) -> Scenario
{
    let start = PreciseTime::now();

    let mut results = vec!();
    for i in 1..6 {
        let creator1 = || -> Box<HasScenario> { Box::new(case1_scenario(i)) };
        let interaction1 = Obstacles::none_significant(i);
        let batch1 = time_batch(creator1, NUM_RUNS);
        let results1 = ObstacleBatch::new(interaction1, batch1);
        results.push(results1);

        let creator2 = || -> Box<HasScenario> { Box::new(case2_scenario(i)) };
        let interaction2 = Obstacles::all_significant(i);
        let batch2 = time_batch(creator2, NUM_RUNS);
        let results2 = ObstacleBatch::new(interaction2, batch2);
        results.push(results2);
    }
    write_results(&results);

    let runtime = start.to(PreciseTime::now()).num_seconds();
    println!("Total time: {} seconds", runtime);
}
