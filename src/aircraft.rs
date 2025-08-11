use serde::Deserialize;
use yaml_rust::Yaml;

// --- 構造体定義 --- start
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[derive(Clone)]
struct AircraftData {
    now: f64,
    messages: i64,
    aircraft: Vec<Aircraft>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[derive(Clone)]
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

pub struct Watcher {
    check_flights: Vec<String>,
}

impl Watcher {
    pub fn new(value: Vec<&str>) -> Self {
        println!("Check Flights: {:?}", value);
        Self {
            check_flights: value.into_iter().map(String::from).collect(),
        }
    }

    fn trim_flight(&self, aircraft: &Aircraft) -> Aircraft {
        Aircraft {
            flight: aircraft.flight.as_ref().map(|s| s.trim().to_string()),
            ..(*aircraft).clone()
        }
    }

    fn is_check_flight(&self, aircraft: &Aircraft) -> bool {
        aircraft.flight
            .as_ref() // &Option<String> から Option<&String> に変換
            .map_or(
                false, // OptionがNoneの場合 false
                |s| self.check_flights.iter().any(|item| item == s) // リストに一致する要素があるかチェック
            )
    }

    pub fn detection(&self, json_str: &str, config: &Yaml) {
        match serde_json::from_str::<AircraftData>(&json_str) {
            Ok(data) => {
                for mut aircraft in data.aircraft {
                    // 便名の前後の空白を削除
                    aircraft = self.trim_flight(&aircraft);

                    // 便名 高度 緯度 経度 いずれかがない場合はスキップ
                    if
                        !aircraft.flight.is_some() ||
                        !aircraft.alt_baro.is_some() ||
                        !aircraft.lat.is_some() ||
                        !aircraft.lon.is_some()
                    {
                        continue;
                    }
                    let message_content = format!(
                        "機体: {}, 便名: {:?}, 高度: {:?}, 緯度経度: ({:?}, {:?})",
                        aircraft.hex,
                        aircraft.flight,
                        aircraft.alt_baro,
                        aircraft.lat,
                        aircraft.lon
                    );
                    println!("{}", message_content);

                    // 監視対象の便名でない場合はスキップ
                    if !self.is_check_flight(&aircraft) {
                        continue;
                    }
                }
            }
            Err(e) => {
                eprintln!("JSON parsing failed: {}", e);
            }
        }
    }
}
