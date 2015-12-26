use super::time::PreciseTime;

// Contains details and results of a batch of benchmarks.
#[derive(Serialize, Deserialize)]
pub struct Batch {
    num_runs: u32,
    run_times: Vec<i64>
}

// Bundles a batch with a label.
#[derive(Serialize, Deserialize)]
pub struct LabelledBatch {
    pub label: String,
    batch: Batch
}

impl LabelledBatch {
    // Convenience function that returns a LabelledBatch.
    pub fn new(label: String, batch: Batch) -> LabelledBatch {
        LabelledBatch { label: label, batch: batch }
    }
}

// For structs that execute scenarios.
pub trait HasScenario {
    fn run(&mut self);
}

// Runs a series of tests on scenarios generated using the provided function.
pub fn time_batch<F>(creator: F, num_runs: u32, num_batches: u32) -> Batch
    where F: Fn() -> Box<HasScenario>
{
    let mut timing = vec!();
    for _ in 0..num_batches {
        let mut scenarios: Vec<Box<HasScenario>> = vec!();
        for _ in 0..num_runs { scenarios.push(creator()) };
        let start = PreciseTime::now();
        for s in scenarios.iter_mut() { s.run() };
        let elapsed = start.to(PreciseTime::now());
        timing.push(elapsed.num_microseconds().unwrap())
    };
    Batch { num_runs: num_runs, run_times: timing }
}
