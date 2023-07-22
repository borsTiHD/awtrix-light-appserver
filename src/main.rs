use std::thread;
use std::time::Duration;

// Import apps
use crate::apps::pikachu;
use crate::apps::clock;

pub mod apps;

// Config
const SLEEP_DURATION: u64 = 1; // Specify the sleep duration in seconds

// Main function
fn main() {
    println!("Starting app server...");

    // Init apps
    pikachu::init();
    clock::init();

    // This loop is added to prevent the main thread from exiting immediately.
    // Since the `init` function spawns a new thread, without this loop, the main
    // thread would exit, and the program would terminate.
    loop {
        // You can add any code you need to run in the main thread here.
        // For example, you can add a termination condition for the loop if needed.

        // Introduce a delay of 1 second between each iteration
        thread::sleep(Duration::from_secs(SLEEP_DURATION));
    }
}