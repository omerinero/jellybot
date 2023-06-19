pub mod api {
    use crate::env_loader::TELEGRAM_BOT_API_KEY;
    use reqwest::header::CONTENT_TYPE;
    use reqwest::{Body, Client};

    pub async fn executeRequest(body: String, endpoint: String) {
        Client::new()
            .post(format!(
                "https://api.telegram.org/bot{}/{}",
                TELEGRAM_BOT_API_KEY(),
                endpoint
            ))
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .send()
            .await
            .unwrap();
    }
}
