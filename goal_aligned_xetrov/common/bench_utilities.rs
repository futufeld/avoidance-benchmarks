use super::types::Scenario;

extern crate utilities;
use utilities::handler::*;
use utilities::constants::*;

// Invokes benchmark handler using the specified scenario type and number of
// obstacles. Returns a LabelledBatch with the given label.
pub fn time_case<F>(label: String, f: F, num_obstacles: u32) -> LabelledBatch
    where F: Fn(u32) -> Scenario
{
    let creator = || -> Box<HasScenario> { Box::new(f(num_obstacles)) };
    LabelledBatch::new(label, time_batch(creator, NUM_RUNS, NUM_BATCHES))
}
