pub mod components;
pub mod rest;

use rand::distributions::Alphanumeric;
use rand::Rng;

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
