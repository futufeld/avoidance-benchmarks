#![feature(custom_derive, plugin, test)]
#![plugin(serde_macros)]

extern crate getopts;
extern crate linalg;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate test;
extern crate time;

pub mod bench_utilities;
pub mod rng_utilities;
pub mod test_utilities;
pub mod types;
pub mod utilities;
