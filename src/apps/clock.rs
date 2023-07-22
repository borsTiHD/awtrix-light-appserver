use std::thread;
use std::time::Duration;
use chrono::prelude::*;

use awtrix_light_appserver::create_app;
use awtrix_light_appserver::update_app;

// Config
const SLEEP_DURATION: u64 = 30; // Specify the sleep duration in seconds

// Init function
pub fn init() {
    // Creating app
    println!("Init clock app...");
    create_app("modern_clock").unwrap();
    update_app("modern_clock", "New Clock", 1082).unwrap();

    // Thread
    let handle = thread::spawn(|| main()); // Spawn a new thread that runs the main function
    handle.join().unwrap(); // Wait for the thread to finish (this won't happen in this case because it's an endless loop)
}

// Main function
fn main() {
    loop {
        update(); // Update function

        // Introduce a delay between each iteration
        thread::sleep(Duration::from_secs(SLEEP_DURATION));
    }
}

// Update function
fn update() {
    // Get the current time in the local timezone
    // Format the time as "HH:MM"
    let local_time: DateTime<Local> = Local::now();
    let formatted_time: String = local_time.format("%H:%M").to_string();

    // Update app with current time
    update_app("modern_clock", &formatted_time, 1082).unwrap();
}
