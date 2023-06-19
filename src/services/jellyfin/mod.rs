mod responses;

pub mod api {
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
    use reqwest::Client;

    use crate::env_loader::JELLYFIN_API_KEY;
    use crate::services::jellyfin::responses::SeriesIdResponse;

    pub fn getPrimaryImage(serverUrl: &String, itemId: &String) -> String {
        return format!("{}/Items/{}/Images/Primary", serverUrl, itemId);
    }

    pub async fn getPrimaryImageOfAnEpisode(
        serverUrl: &String,
        userId: &String,
        itemId: &String,
    ) -> String {
        let url = format!("{}/Users/{}/Items/{}", serverUrl, userId, itemId);

        let header_token =
            format!(r#"MediaBrowser Token="{}""#, JELLYFIN_API_KEY().to_owned()).to_owned();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            HeaderName::from_static("x-emby-authorization"),
            header_token.parse().unwrap(),
        );

        Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await
            .unwrap()
            .json::<SeriesIdResponse>()
            .await
            .unwrap()
            .seriesId
    }
}
