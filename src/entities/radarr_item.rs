use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarrItem {
    pub movie: Movie,
    pub remote_movie: RemoteMovie,
    pub movie_file: MovieFile,
    pub is_upgrade: bool,
    pub download_client: String,
    pub download_client_type: String,
    pub download_id: String,
    pub event_type: String,
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub year: i64,
    pub release_date: String,
    pub folder_path: String,
    pub tmdb_id: i64,
    pub imdb_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteMovie {
    pub tmdb_id: i64,
    pub imdb_id: String,
    pub title: String,
    pub year: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieFile {
    pub id: i64,
    pub relative_path: String,
    pub path: String,
    pub quality: String,
    pub quality_version: i64,
    pub scene_name: String,
    pub indexer_flags: String,
    pub size: i64,
    pub date_added: String,
    pub media_info: MediaInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaInfo {
    pub audio_channels: f64,
    pub audio_codec: String,
    pub audio_languages: Vec<String>,
    pub height: i64,
    pub width: i64,
    pub subtitles: Vec<Value>,
    pub video_codec: String,
    pub video_dynamic_range: String,
    pub video_dynamic_range_type: String,
}
