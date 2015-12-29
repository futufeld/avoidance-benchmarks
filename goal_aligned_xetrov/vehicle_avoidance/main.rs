extern crate vehicle_avoidance;
use vehicle_avoidance::scenarios::*;

extern crate utilities;
use utilities::utilities::*;

extern crate common;
use common::bench_utilities::time_case;

fn main() {
    let run = || {
        let mut results = vec!();
        for i in 1..6 {
            let case1_label = format!("Insignificant {} - 0 Significant", i);
            results.push(time_case(case1_label, case1_scenario, i));

            let case2_label = format!("Insignificant 0 - {} Significant", i);
            results.push(time_case(case2_label, case2_scenario, i));
        }
        write_results(&results);
    };
    println!("Total time: {} seconds", time_execution_seconds(run));
}
