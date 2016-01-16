use super::rand::distributions::{IndependentSample, Range};
use super::rand::thread_rng;

use std::f64::consts::PI;

// Returns a random value between 0f64 and 1f64 using the thread's random
// number generator.
pub fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

// Returns a random value between zero and two PI using the thread's random
// number generator.
pub fn random_tau() -> f64 {
    Range::new(0f64, 2f64 * PI).ind_sample(&mut thread_rng())
}

// Returns a random value between 0.1f64 and 0.9f64 using the thread's random
// number generator. Useful when numbers near the unity boundary are
// undesirable.
pub fn random_margin() -> f64 {
    Range::new(0.1f64, 0.9f64).ind_sample(&mut thread_rng())
}
