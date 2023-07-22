use std::thread;
use std::time::Duration;

// Config
const SLEEP_DURATION: u64 = 1; // Specify the sleep duration in seconds

// Init function
pub fn init() {
    // Init pikachu app
    println!("Init pikachu app...");

    // Spawn a new thread that runs the loop function
    let handle = thread::spawn(|| main());

    // Wait for the thread to finish (this won't happen in this case because it's an endless loop)
    handle.join().unwrap();
}

// Main function
fn main() {
    loop {
        // Update
        update();

        // Introduce a delay of 1 second between each iteration
        thread::sleep(Duration::from_secs(SLEEP_DURATION));
    }
}

// Update function
fn update() {
    // Print to console
    println!("Update pikachu app...");
}
