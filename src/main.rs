use std::time::Duration;

mod aircraft;
mod config;
mod discord;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load();

    let url = config["TAR1090_URL"].as_str().expect("TAR1090_URL not found in config");
    let interval = config["TAR1090_INTERVAL"]
        .as_i64()
        .expect("TAR1090_INTERVAL not found in config");
    let webhook = config["DISCORD_WEBHOOK_URL"]
        .as_str()
        .expect("DISCORD_WEBHOOK_URL not found in config");
    let cooldown =
        config["RE_NOTICE_MINUTE"].as_f64().expect("RE_NOTICE_MINUTE not found in config") * 60.0; // 秒単位 変換
    // Yaml::Array 型かどうか確認して Vec<String> に変換
    let flight_vec: Vec<&str> = config["CHECK_FLIGHTS"]
        .as_vec()
        .expect("CHECK_FLIGHTS not found in config")
        .iter()
        .filter_map(|item| item.as_str()) // .map(String::from) を削除
        .collect();

    let client = reqwest::Client::new();
    let mut waiter = tokio::time::interval(Duration::from_secs(interval as u64));
    let watcher = aircraft::Watcher::new(flight_vec, webhook, &cooldown);
    loop {
        waiter.tick().await;

        match client.get(url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    match res.text().await {
                        Ok(body_text) => {
                            watcher.detection(&body_text).await;
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
