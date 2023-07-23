use awtrix_light_appserver::create_app;
use awtrix_light_appserver::update_app;

// App config
const APP_NAME: &str = "pikachu";
const APP_ICON: i32 = 5588; // Specify the app icon

// Init function
pub fn init() {
    // Creating app
    println!("--------------------------");
    println!("Init app: {}", APP_NAME);
    create_app(APP_NAME).unwrap();
    update_app(APP_NAME, "Pika Pi!", APP_ICON, None).unwrap();
}
