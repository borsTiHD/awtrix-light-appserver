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
// The payload parameter is optional
// If you want to use the payload parameter, you can pass a JSON string to it
// If you don't want to use the payload parameter, you can pass None to it
// If you pass None to the payload parameter, the function will create a payload with the app_text and app_icon
// If you pass a JSON string to the payload parameter, the function will use the value of the parameter and ignore the app_text and app_icon parameters
#[tokio::main]
pub async fn update_app(app_name: &str, app_text: &str, app_icon: i32, payload: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the optional payload parameter is set
    // If it is set, use the value of the parameter
    // If it is not set, create a payload with the app_text and app_icon
    // Either way, serialize the payload into a JSON string and set it to the json_payload variable
    let json_payload: String = if let Some(value) = payload {
        // Set the payload string to the value of the optional parameter
        value.to_string()
    } else {
        // Create an instance of the Payload struct with your desired values
        let payload: Payload = Payload {
            text: app_text.to_string(),
            icon: app_icon
        };
    
        // Serialize the payload into a JSON string
        serde_json::to_string(&payload).unwrap()
    };

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