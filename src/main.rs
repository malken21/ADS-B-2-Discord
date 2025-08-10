use serde::Deserialize;
use std::time::Duration;

// --- 構造体定義 --- start
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 使わないフィールドがあっても警告を出さない
struct AircraftData {
    now: f64,
    messages: i64,
    aircraft: Vec<Aircraft>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 使わないフィールドがあっても警告を出さない
struct Aircraft {
    hex: String,
    flight: Option<String>,
    squawk: Option<String>,
    alt_baro: Option<serde_json::Value>,
    lat: Option<f64>,
    lon: Option<f64>,
    gs: Option<f64>,
    track: Option<f64>,
    category: Option<String>,
    messages: i64,
    rssi: f64,
}
// --- 構造体定義 --- end

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://192.168.0.55:8080/data/aircraft.json";
    let client = reqwest::Client::new();
    let mut interval = tokio::time::interval(Duration::from_secs(3));
    loop {
        interval.tick().await;

        match client.get(url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    match res.text().await {
                        Ok(body_text) => {
                            // テキストから新しいstructへのパースを試みる
                            match serde_json::from_str::<AircraftData>(&body_text) {
                                Ok(data) => {
                                    // 検出したすべての航空機情報を簡単に出力する
                                    for aircraft in data.aircraft {
                                        println!(
                                            "機体: {}, 便名: {:?}, 高度: {:?}, 緯度経度: ({:?}, {:?})",
                                            aircraft.hex,
                                            aircraft.flight.map(|s| s.trim().to_string()), // 前後の空白を削除
                                            aircraft.alt_baro,
                                            aircraft.lat,
                                            aircraft.lon
                                        );
                                    }
                                }
                                Err(e) => {
                                    eprintln!("JSON parsing failed: {}", e);
                                }
                            }
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
