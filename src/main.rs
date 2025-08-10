use std::time::Duration;

mod aircraft;
use aircraft::aircraft;

let url = env::var("TAR1090_URL").unwrap_or_else(|_| "http://localhost:8080/data/aircraft.json".to_string());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut interval = tokio::time::interval(Duration::from_secs(3));
    loop {
        interval.tick().await;

        match client.get(url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    match res.text().await {
                        Ok(body_text) => {
                            aircraft(&body_text);
                        }
                        Err(e) => eprintln!("Failed to read response body: {}", e),
                    }
                } else {
                    eprintln!("Request Failed: {}", res.status());
                    if let Ok(body) = res.text().await {
                        eprintln!("Response from the server: {}", body);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to send request: {}", e);
            }
        }
    }
}
