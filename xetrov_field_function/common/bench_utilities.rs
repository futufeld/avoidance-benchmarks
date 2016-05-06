extern crate utilities;
use utilities::bench_utilities::time_batch;
use utilities::types::{HasScenario, Obstacles};
use utilities::utilities::write_results;

extern crate time;
use self::time::PreciseTime;

// Runs benchmarks and saves results to a file specified on the command line.
pub fn run_benchmarks<F>(scenario: F)
    where F: Fn(&Obstacles) -> Option<Box<HasScenario>>
{
    let start = PreciseTime::now();

    let creator = |o: &Obstacles| -> Box<HasScenario> { scenario(o).unwrap() };

    let mut results = vec!();
    for i in 1..6 {
        let interaction1 = Obstacles::new(i, 0u32);
        results.push(time_batch(&interaction1, |x| creator(x)));
        let interaction2 = Obstacles::new(0u32, i);
        results.push(time_batch(&interaction2, |x| creator(x)));
    }
    write_results(&results);

    let runtime = start.to(PreciseTime::now()).num_seconds();
    println!("Total time: {} seconds", runtime);
}
