use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SeriesIdResponse {
    #[serde(rename = "SeriesId")]
    pub seriesId: String, //SeriesId
}
