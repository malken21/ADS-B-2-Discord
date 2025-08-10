use std::time::Duration;

mod util;
use util::get_env;

mod aircraft;
use aircraft::aircraft;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = get_env("TAR1090_URL", "http://localhost:8080/data/aircraft.json");
    let mut interval = tokio::time::interval(
        Duration::from_secs(get_env("TAR1090_INTERVAL", "5").parse().unwrap())
    );

    let client = reqwest::Client::new();
    loop {
        interval.tick().await;

        match client.get(&url).send().await {
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
