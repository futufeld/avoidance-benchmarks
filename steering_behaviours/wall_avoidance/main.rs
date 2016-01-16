extern crate wall_avoidance;
use wall_avoidance::scenarios::*;

extern crate utilities;
use utilities::handler::*;
use utilities::utilities::*;

// Starts benchmarks and writes results to file.
fn main() {
    let run = || {
        let creator = |o: &Obstacles| -> Box<HasScenario> {
            scenario_with_obstacles(o).unwrap()
        };

        let mut interactions = vec!();
        interactions.push(Obstacles::new(1u32, 0u32));
        interactions.push(Obstacles::new(4u32, 0u32));
        interactions.push(Obstacles::new(9u32, 0u32));
        interactions.push(Obstacles::new(0u32, 1u32));
        interactions.push(Obstacles::new(2u32, 2u32));
        interactions.push(Obstacles::new(6u32, 3u32));

        let mut results = vec!();
        for interaction in interactions.iter() {
            results.push(time_batch(&interaction, |x| creator(x)));
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
