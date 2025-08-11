use serde::Serialize;

#[derive(Serialize)]
struct DiscordWebhookPayload<'a> {
    content: &'a str,
}

// この関数は Webhook URL とメッセージ内容を引数に取る
pub async fn send_discord_webhook(url: &str, content: &str) -> Result<(), reqwest::Error> {
    // 送信するペイロードを作成
    let payload = DiscordWebhookPayload {
        content,
    };

    // HTTPクライアントを作成してリクエストを送信
    let client = reqwest::Client::new();
    let response = client.post(url).json(&payload).send().await?;

    // ステータスコードをチェックし、エラーなら詳細を返す
    response.error_for_status()?;

    println!("Webhookメッセージを正常に送信しました。");
    Ok(())
}
