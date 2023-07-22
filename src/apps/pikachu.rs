use awtrix_light_appserver::create_app;

// Init function
pub fn init() {
    // Creating app
    println!("Init pikachu app...");
    create_app("pikachu", "Pika Pi!", 5588).unwrap();
}