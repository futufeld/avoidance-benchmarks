extern crate wall_avoidance;
use wall_avoidance::scenarios::scenario_with_obstacles;

extern crate common;
use common::bench_utilities::run_benchmarks;

fn main() {
    run_benchmarks(scenario_with_obstacles);
}
