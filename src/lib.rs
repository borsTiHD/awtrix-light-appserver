use std::env;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde_derive::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Payload {
    text: String,
    icon: i32,
}

// Endpoints
const ENDPOINT_CUSTOM: &str = "/custom";

// Function to create an app
#[tokio::main]
pub async fn create_app(app_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a reqwest client
    let client: Client = Client::new();

    // The URL to which you want to send the POST request
    let base_url = env::var("BASE_URL").unwrap();
    let url: String = format!("{}{}", base_url, ENDPOINT_CUSTOM);

    // Send the POST request with the JSON payload as the body
    let response: reqwest::Response = client
        .post(url)
        .query(&[("name", app_name)]) // Add the query parameter
        .header(CONTENT_TYPE, "application/json")
        .body("") // Empty body
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Request was successful!");
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    // Read the response body as a string
    let response_body: String = response.text().await?;
    println!("Response body: {}", response_body);

    Ok(())
}

// Function to update an app
#[tokio::main]
pub async fn update_app(app_name: &str, app_text: &str, app_icon: i32) -> Result<(), Box<dyn std::error::Error>> {
    // Create an instance of the Payload struct with your desired values
    let payload = Payload {
        text: app_text.to_string(),
        icon: app_icon
    };

    // Serialize the payload into a JSON string
    let json_payload: String = serde_json::to_string(&payload).unwrap();

    // Create a reqwest client
    let client: Client = Client::new();

    // The URL to which you want to send the POST request
    let base_url = env::var("BASE_URL").unwrap();
    let url: String = format!("{}{}", base_url, ENDPOINT_CUSTOM);

    // Send the POST request with the JSON payload as the body
    let response: reqwest::Response = client
        .post(url)
        .query(&[("name", app_name)]) // Add the query parameter
        .header(CONTENT_TYPE, "application/json")
        .body(json_payload) // Or use .json(&json_value) if using Value directly
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Request was successful!");
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    // Read the response body as a string
    let response_body: String = response.text().await?;
    println!("Response body: {}", response_body);

    Ok(())
}