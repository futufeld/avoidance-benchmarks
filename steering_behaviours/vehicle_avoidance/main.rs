extern crate vehicle_avoidance;
use vehicle_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let creator = |o: &Obstacles| -> Box<HasScenario> {
            scenario_with_obstacles(o).unwrap()
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
