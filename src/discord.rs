use serde::Serialize;

#[derive(Serialize)]
struct DiscordWebhookPayload<'a> {
    content: &'a str,
}

pub struct DiscordWebhook {
    url: String,
}

impl DiscordWebhook {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub async fn send_discord_webhook(&self, content: &str) -> Result<(), reqwest::Error> {
        // 送信するペイロード 作成
        let payload = DiscordWebhookPayload {
            content,
        };
        // HTTPクライアントを作成してリクエスト 送信
        let client = reqwest::Client::new();
        let response = client.post(&self.url).json(&payload).send().await?;
        // ステータスコードをチェック エラーなら詳細を返す
        response.error_for_status()?;
        Ok(())
    }
}
