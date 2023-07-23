use chrono::prelude::*;
use chrono::Local;
use awtrix_light_appserver::create_app;
use awtrix_light_appserver::update_app;

// App config
const APP_NAME: &str = "pikachu";
const APP_ICON: i32 = 5588; // Specify the app icon

// Init function
pub fn init() {
    // Log the init
    let current_time: DateTime<Local> = Local::now();
    let formatted_time: String = current_time.format("%H:%M").to_string();
    println!("--------------------------");
    println!("[{}] | Init app: {}", formatted_time, APP_NAME);

    // Creating app
    create_app(APP_NAME).unwrap();
    update_app(APP_NAME, "Pika Pi!", APP_ICON, None).unwrap();
}
