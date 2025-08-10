use serde::Deserialize;

// --- 構造体定義 --- start
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct AircraftData {
    now: f64,
    messages: i64,
    aircraft: Vec<Aircraft>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

pub fn aircraft(json_str: &str) {
    // テキストから新しいstructへのパースを試みる
    match serde_json::from_str::<AircraftData>(&json_str) {
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
