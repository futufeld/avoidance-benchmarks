extern crate wall_avoidance;
use wall_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;
use utilities::constants::*;

// Runs benchmarks for scenarios involving insignificant obstacles.
fn case1_benchmarks() -> Vec<ObstacleBatch> {
    let mut results = vec!();

    // Spear scenarios.
    let creator1 = || -> Box<HasScenario> {
        Box::new(case1_scenario(FeelerShape::Spear))
    };
    let interaction1 = Obstacles::none_significant(1);
    let batch1 = time_batch(creator1, NUM_RUNS, NUM_BATCHES);
    let results1 = ObstacleBatch::new(interaction1, batch1);
    results.push(results1);

    // Fork scenarios.
    let creator2 = || -> Box<HasScenario> {
        Box::new(case1_scenario(FeelerShape::Fork))
    };
    let interaction2 = Obstacles::none_significant(4);
    let batch2 = time_batch(creator2, NUM_RUNS, NUM_BATCHES);
    let results2 = ObstacleBatch::new(interaction2, batch2);
    results.push(results2);

    // Trident scenarios.
    let creator3 = || -> Box<HasScenario> {
        Box::new(case1_scenario(FeelerShape::Trident))
    };
    let interaction3 = Obstacles::none_significant(9);
    let batch3 = time_batch(creator3, NUM_RUNS, NUM_BATCHES);
    let results3 = ObstacleBatch::new(interaction3, batch3);
    results.push(results3);

    results
}

// Runs benchmarks for scenarios involving significant obstacles.
fn case2_benchmarks() -> Vec<ObstacleBatch> {
    let mut results = vec!();

    // Spear scenarios.
    let creator1 = || -> Box<HasScenario> {
        Box::new(case2_scenario(FeelerShape::Spear))
    };
    let interaction1 = Obstacles::all_significant(1);
    let batch1 = time_batch(creator1, NUM_RUNS, NUM_BATCHES);
    let results1 = ObstacleBatch::new(interaction1, batch1);
    results.push(results1);

    // Fork scenarios.
    let creator2 = || -> Box<HasScenario> {
        Box::new(case2_scenario(FeelerShape::Fork))
    };
    let interaction2 = Obstacles::new(4, 2, 2);
    let batch2 = time_batch(creator2, NUM_RUNS, NUM_BATCHES);
    let results2 = ObstacleBatch::new(interaction2, batch2);
    results.push(results2);

    // Trident scenarios.
    let creator3 = || -> Box<HasScenario> {
        Box::new(case2_scenario(FeelerShape::Trident))
    };
    let interaction3 = Obstacles::new(9, 6, 3);
    let batch3 = time_batch(creator3, NUM_RUNS, NUM_BATCHES);
    let results3 = ObstacleBatch::new(interaction3, batch3);
    results.push(results3);

    results
}

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let mut results = case1_benchmarks();
        results.extend(case2_benchmarks());
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
