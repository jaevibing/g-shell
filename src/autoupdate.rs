extern crate reqwest;
extern crate tokio;
use reqwest::Error;

pub async fn checkForUpdate() -> Result<String, Error> {
    // Create a client to make the request
    let client = reqwest::Client::new();

    // The URL you want to send the GET request to
    let url = "https://raw.githubusercontent.com/jaevibing/g-shell/master/src/VERSION";

    // Send the GET request and await the response
    let response = client.get(url).send().await?;

    let mut response_text = String::new();

    // Check if the request was successful (status code in the 2xx range)
    if response.status().is_success() {
        // Read the response body as a string
        response_text = response.text().await?;
    } else {
        // Handle error cases, e.g., print the status code and reason phrase
        println!(
            "Request failed with status code: {} - {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("Unknown Reason")
        );
    }

    Ok(response_text)
}

pub fn update(){

}