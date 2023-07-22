use dotenv::dotenv;

// Import apps
use crate::apps::pikachu;
use crate::apps::clock;

pub mod apps;

// Main function
fn main() {
    println!("Starting app server...");

    // Load the environment variables from the ".env" file
    dotenv().ok();

    // Init apps
    pikachu::init();
    clock::init();
}