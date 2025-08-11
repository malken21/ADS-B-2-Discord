use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{ Duration, Instant };
use reqwest::header;
use reqwest::Client;

pub struct DiscordWebhook {
    url: String,
    cooldown: Duration,
    payload_template: String,
    // 送信タイムスタンプを記録するためのHashMap
    // Mutexでラップして、&self経由でもスレッドセーフに変更できるようにする
    timestamps: Mutex<HashMap<String, Instant>>,
}

impl DiscordWebhook {
    pub fn new(url: &str, cooldown: &f64, payload_template: &str) -> Self {
        Self {
            url: url.to_string(),
            cooldown: Duration::from_secs_f64(*cooldown),
            payload_template: payload_template.to_string(),
            timestamps: Mutex::new(HashMap::new()),
        }
    }

    pub async fn send(
        &self,
        hex: &str,
        flight: &str,
        alt: &f64,
        lat: &f64,
        lon: &f64
    ) -> Result<(), reqwest::Error> {
        // クールダウンチェック
        {
            // Mutexのロックガードのスコープを限定
            let timestamps = self.timestamps.lock().unwrap(); // Mutexをロック
            if let Some(last_sent) = timestamps.get(hex) {
                // 前回送信からの経過時間がクールダウン時間より短い場合
                if last_sent.elapsed() < self.cooldown {
                    // 何もせず正常終了
                    return Ok(());
                }
            }
        } // ここでロックが自動的に解除

        let payload = self.payload_template
            .replace("{hex}", hex)
            .replace("{flight}", flight)
            .replace("{alt}", &alt.to_string())
            .replace("{lat}", &lat.to_string())
            .replace("{lon}", &lon.to_string());

        println!("機体: {}, 便名: {}, 高度: {}, 緯度経度: ({}, {})", hex, flight, alt, lat, lon);

        let client = Client::new();
        let response = client
            .post(&self.url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send().await?;

        // ステータスコードがエラーなら詳細を返す
        response.error_for_status()?;

        // 送信成功時 タイムスタンプ 更新
        {
            let mut timestamps = self.timestamps.lock().unwrap();
            // 現在時刻 記録(または更新)
            timestamps.insert(hex.to_string(), Instant::now());
        }

        Ok(())
    }
}
