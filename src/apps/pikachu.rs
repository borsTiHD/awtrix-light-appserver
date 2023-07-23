use awtrix_light_appserver::create_app;
use awtrix_light_appserver::update_app;

// Init function
pub fn init() {
    // Creating app
    println!("Init pikachu app...");
    create_app("pikachu").unwrap();
    update_app("pikachu", "Pika Pi!", 5588).unwrap();
}
