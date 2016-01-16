use super::test::black_box;
use super::time::PreciseTime;

// Number of scenarios to run for each benchmark.
#[allow(dead_code)]
pub const NUM_RUNS: u32 = 10_000;

// Contains details about obstacle interactions.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Obstacles { total:         u32
                     , insignificant: u32
                     , significant:   u32 }

impl Obstacles {
    // Returns an Obstacles populated by the given values.
    pub fn new(total: u32, insignificant: u32, significant: u32) -> Obstacles {
        Obstacles { total:         total
                  , insignificant: insignificant
                  , significant:   significant }
    }

    // Specialisation of new that indicates all obstacles are insignificant.
    pub fn none_significant(num_obstacles: u32) -> Obstacles {
        Obstacles::new(num_obstacles, num_obstacles, 0u32)
    }

    // Specialisation of new that indicates all obstacles are significant.
    pub fn all_significant(num_obstacles: u32) -> Obstacles {
        Obstacles::new(num_obstacles, 0u32, num_obstacles)
    }
}

// Contains details and results of a batch of benchmarks.
#[derive(Serialize, Deserialize)]
pub struct Batch { num_runs:  u32
                 , run_times: Vec<i64> }

// Bundles a batch with a label.
#[derive(Serialize, Deserialize)]
pub struct ObstacleBatch { obstacles: Obstacles
                         , batch:     Batch }

impl ObstacleBatch {
    // Creates an ObstacleBatch from the given values.
    pub fn new(obstacles: Obstacles, batch: Batch) -> ObstacleBatch {
        ObstacleBatch { obstacles: obstacles, batch: batch }
    }
}

// For structs that execute scenarios.
pub trait HasScenario {
    fn run(&mut self);
}

// Runs a series of tests on scenarios generated using the provided function.
pub fn time_batch<F>(creator: F, num_runs: u32) -> Batch
    where F: Fn() -> Box<HasScenario>
{
    let mut scenarios: Vec<Box<HasScenario>> =
        (0..num_runs).map(|_| creator()).collect();

    let mut timing = vec!();
    for s in scenarios.iter_mut() {
        let start = PreciseTime::now();
        black_box(s.run());
        let elapsed = start.to(PreciseTime::now());
        timing.push(elapsed.num_nanoseconds().unwrap());
    }

    Batch { num_runs: num_runs, run_times: timing }
}
