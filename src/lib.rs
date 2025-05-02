pub mod components;
pub mod layouts;
pub mod rest;
pub mod toast_notifications;

use rand::distributions::Alphanumeric;
use rand::Rng;

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
