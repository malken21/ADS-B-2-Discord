use serde::Deserialize;

use crate::discord::DiscordWebhook;

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
    alt_baro: Option<f64>,
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
    discord: DiscordWebhook,
}

impl Watcher {
    pub fn new(value: Vec<&str>, discord_webhook_url: &str, cooldown: &f64) -> Self {
        println!("Check Flights: {:?}", value);
        Self {
            check_flights: value.into_iter().map(String::from).collect(),
            discord: DiscordWebhook::new(discord_webhook_url, cooldown),
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

    pub async fn detection(&self, json_str: &str) {
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

                    // 監視対象の便名でない場合はスキップ
                    //if !self.is_check_flight(&aircraft) {
                    //    continue;
                    //}

                    self.discord
                        .send(
                            &aircraft.hex,
                            &aircraft.flight.unwrap(),
                            &aircraft.alt_baro.unwrap(),
                            &aircraft.lat.unwrap(),
                            &aircraft.lon.unwrap()
                        ).await
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to send Discord webhook: {}", e);
                        });
                }
            }
            Err(e) => {
                eprintln!("JSON parsing failed: {}", e);
            }
        }
    }
}
