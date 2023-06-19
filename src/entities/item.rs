use serde::Deserialize;

#[derive(Deserialize)]
pub struct Item {
    #[serde(default, rename = "ItemId")]
    pub id: String, //ItemId
    #[serde(default, rename = "ServerUrl")]
    pub serverUrl: String, //ServerUrl
    #[serde(default, rename = "Name")]
    pub name: String, //Name
    #[serde(rename = "ItemType")]
    pub itemType: ItemType, //ItemType
    #[serde(default, rename = "SeriesName")]
    pub seriesName: String, //SeriesName
    #[serde(default, rename = "SeasonNumber00")]
    pub seasonNumber: String, //SeasonNumber00
    #[serde(default, rename = "EpisodeNumber00")]
    pub episodeNumber: String, //EpisodeNumber00
    #[serde(default, rename = "UserId")]
    pub userId: String, //UserId
    #[serde(default, rename = "Provider_imdb")]
    pub imdb: String, //Provider_imdb,
    #[serde(default, rename = "Provider_tvdb")]
    pub tvdb: String, //Provider_tvdb,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum ItemType {
    #[serde(rename = "Movies")]
    Movies,
    #[serde(rename = "Movie")]
    Movie,
    #[serde(rename = "Series")]
    Series,
    #[serde(rename = "Episodes")]
    Episodes,
    #[serde(rename = "Episode")]
    Episode,
    #[serde(rename = "Season")]
    Season,
    #[serde(rename = "Seasons")]
    Seasons,
}
