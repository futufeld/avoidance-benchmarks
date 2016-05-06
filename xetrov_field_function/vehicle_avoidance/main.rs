extern crate vehicle_avoidance;
use vehicle_avoidance::scenarios::scenario_with_obstacles;

extern crate common;
use common::bench_utilities::run_benchmarks;

fn main() {
    run_benchmarks(scenario_with_obstacles);
}
