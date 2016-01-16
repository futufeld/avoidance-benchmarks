extern crate disc_avoidance;
use disc_avoidance::scenarios::scenario_with_obstacles;

extern crate common;
use common::bench_utilities::run_benchmarks;

fn main() {
    run_benchmarks(scenario_with_obstacles);
}
