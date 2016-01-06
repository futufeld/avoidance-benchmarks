extern crate obstacle_avoidance;
use obstacle_avoidance::scenarios::{case1_scenario, case2_scenario};

extern crate common;
use common::bench_utilities::run_benchmarks;

fn main() {
    run_benchmarks(case1_scenario, case2_scenario);
}
