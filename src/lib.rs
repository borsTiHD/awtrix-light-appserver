use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde_derive::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Payload {
    text: String,
    icon: i32,
}

#[tokio::main]
pub async fn create_app(app_name: &str, app_text: &str, app_icon: i32) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating app {}...", app_name);

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
    let url: &str = "http://192.168.2.50/api/custom";

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