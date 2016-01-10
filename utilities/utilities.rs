use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::getopts::Options;
use super::handler::ObstacleBatch;
use super::rand::distributions::{IndependentSample, Range};
use super::rand::thread_rng;
use super::serde_json::to_string_pretty;
use super::test::black_box;
use super::time::PreciseTime;

// Returns a random value between 0f64 and 1f64 using the thread's random
// number generator.
pub fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

// Times the execution of the given function in seconds.
pub fn time_execution_seconds<F>(to_execute :F) -> i64
    where F: Fn() -> ()
{
    let start = PreciseTime::now();
    black_box(to_execute());
    start.to(PreciseTime::now()).num_seconds()
}

// Applies basic command-line option functionality.
pub fn get_filepath() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();
    if !matches.free.is_empty() {
        return Some(matches.free[0].clone());
    }

    let brief = format!("Usage: {} FILE", program);
    print!("{}", opts.usage(&brief));
    None
}

// Writes test information to the specified file.
pub fn write_batches(filepath: &Path, batches: &Vec<ObstacleBatch>) {
    let json = to_string_pretty(&batches).unwrap();

    let mut file = match File::create(&filepath) {
        Err(error) =>  panic!( "couldn't create {}: {}"
                             , filepath.display()
                             , Error::description(&error) ),
        Ok(file) => file
    };

    if let Err(error) = file.write_all(json.as_bytes()) {
        panic!( "couldn't write to {}: {}"
              , filepath.display()
              , Error::description(&error) );
    }
}

// Convenience function for writing ObstacleBatch data to user-specified file.
pub fn write_results(results: &Vec<ObstacleBatch>) {
    if let Some(filestring) = get_filepath() {
        let filepath = Path::new(&filestring);
        write_batches(&filepath, results);
    }
}
