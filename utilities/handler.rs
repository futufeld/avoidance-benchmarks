use super::linalg::vector2d::Vec2D;
use super::test::black_box;
use super::time::PreciseTime;

// Number of scenarios to run for each benchmark.
#[allow(dead_code)]
pub const NUM_RUNS: u32 = 10_000;

// For scenarios that are testable without needing to access internals.
pub trait HasScenario {
    fn interactions(&self) -> u32;
    fn avoidance(&self) -> Option<Vec2D>;
}

// Contains details about obstacle interactions.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Obstacles { total:         u32
                     , insignificant: u32
                     , significant:   u32 }

impl Obstacles {
    // Returns an Obstacles populated by the given values.
    pub fn new(insignificant: u32, significant: u32) -> Obstacles {
        Obstacles { total:         insignificant + significant
                  , insignificant: insignificant
                  , significant:   significant }
    }

    // Returns a tuple containing the number of insignificant and significant
    // obstacles.
    pub fn details(&self) -> (u32, u32) {
        (self.insignificant, self.significant)
    }
}

// Contains details and results of a batch of benchmarks.
#[derive(Serialize, Deserialize)]
pub struct Batch { num_runs:  u32
                 , run_times: Vec<i64> }

impl Batch {
    // Creates a Batch from the given values.
    pub fn new(num_runs: u32, run_times: Vec<i64>) -> Batch {
        Batch { num_runs: num_runs, run_times: run_times }
    }
}

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

// Runs a series of tests on scenarios generated using the provided function.
pub fn time_batch<F>(obstacles: &Obstacles, creator: F)
    -> ObstacleBatch
    where F: Fn(&Obstacles) -> Box<HasScenario>
{
    let mut scenarios: Vec<Box<HasScenario>> =
        (0..NUM_RUNS + 100).map(|_| creator(obstacles)).collect();

    let mut count = 0;
    let mut timing = vec!();
    for s in scenarios.iter_mut() {
        let start = PreciseTime::now();
        black_box(s.avoidance());
        let elapsed = start.to(PreciseTime::now());

        count += 1;
        if count >= 100 {
            timing.push(elapsed.num_nanoseconds().unwrap());
        }
    }

    ObstacleBatch::new(obstacles.clone(), Batch::new(NUM_RUNS, timing))
}
