use chrono::prelude::*;
use chrono::{Datelike, Local};
use std::thread;
use std::time::Duration;
use serde_json::{json, Value};

use awtrix_light_appserver::create_app;
use awtrix_light_appserver::update_app;

// App config
const APP_NAME: &str = "modern_clock";
const APP_ICON: i32 = 1082; // Specify the app icon
const SLEEP_DURATION: u64 = 30; // Specify the sleep duration in seconds

// Init function
pub fn init() {
    // Creating app
    println!("--------------------------");
    println!("Init app: {}", APP_NAME);
    create_app(APP_NAME).unwrap();
    update();

    // Thread
    let handle = thread::spawn(|| main()); // Spawn a new thread that runs the main function
    handle.join().unwrap(); // Wait for the thread to finish (this won't happen in this case because it's an endless loop)
}

// Main function
fn main() {
    loop {
        println!("--------------------------");
        println!("Updating app: {}", APP_NAME);    
        update();

        // Introduce a delay between each iteration
        thread::sleep(Duration::from_secs(SLEEP_DURATION));
    }
}

// Update function
fn update() {
    // Get the current time and day
    let now: DateTime<Local> = Local::now();
    let formatted_time: String = now.format("%H:%M").to_string(); // Get the current time in the local timezone - format the time as "HH:MM"
    let current_weekday = now.weekday().num_days_from_monday(); // Get the current weekday

    // Build a JSON array with draw commands, for example: [{"dl": [8, 7, 28, 7, "#FF0000"]}]
    // The draw commands are used to draw a line on the matrix for each day of the week
    // The current weekday will be highlighted with a different color
    let mut draws = vec![
        // json!({"dl": [8, 7, 28, 7, "#FF0000"]}), // Complete line
        json!({"dl": [9, 7, 10, 7, "#8f1f1f"]}), // Mo
        json!({"dl": [12, 7, 13, 7, "#8f1f1f"]}), // Di
        json!({"dl": [15, 7, 16, 7, "#8f1f1f"]}), // Mi
        json!({"dl": [18, 7, 19, 7, "#8f1f1f"]}), // Do
        json!({"dl": [21, 7, 22, 7, "#8f1f1f"]}), // Fr
        json!({"dl": [24, 7, 25, 7, "#8f1f1f"]}), // Sa
        json!({"dl": [27, 7, 28, 7, "#8f1f1f"]}), // So
    ];

    // Update the color code for the current weekday
    let color_code_current_day: Value = json!("#FF0000");
    draws[current_weekday as usize]["dl"][4] = color_code_current_day;

    // Build a JSON object with variables for the payload
    let payload = json!({
        "text": formatted_time.to_string(),
        "icon": APP_ICON,
        "draw": draws
    });

    // Serialize the payload into a JSON string
    let json_payload: String = serde_json::to_string(&payload).unwrap();

    // Update app with current time
    update_app(APP_NAME, "", 0, Some(json_payload.as_str())).unwrap();
}
