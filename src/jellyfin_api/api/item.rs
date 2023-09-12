use anyhow::Result;
use reqwest::Url;
use serde::Deserialize;
use speedate::DateTime;

use crate::{
    config,
    jellyfin_api::{api_client::ApiClient, util::url::httpify},
    utils::datetime_serde::deserialize_datetime_opt,
};

pub fn get_stream_url(server: &config::Server, item_id: &str) -> String {
    Url::parse(&httpify(&server.url))
        .unwrap()
        .join(&format!("Videos/{}/stream?static=true", item_id))
        .unwrap()
        .to_string()
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetItemRes {
    #[serde(rename = "Type")]
    pub item_type: ItemType,
    pub media_sources: Option<Vec<MediaSource>>,
    pub overview: Option<String>,
    pub community_rating: Option<f32>,
    pub official_rating: Option<String>,
    pub genres: Option<Vec<String>>,
    pub status: Option<String>,
    pub production_year: Option<isize>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_datetime_opt")]
    pub premiere_date: Option<DateTime>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_datetime_opt")]
    pub end_date: Option<DateTime>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ItemType {
    AggregateFolder,
    Audio,
    AudioBook,
    BasePluginFolder,
    Book,
    BoxSet,
    Channel,
    ChannelFolderItem,
    CollectionFolder,
    Episode,
    Folder,
    Genre,
    ManualPlaylistsFolder,
    Movie,
    LiveTvChannel,
    LiveTvProgram,
    MusicAlbum,
    MusicArtist,
    MusicGenre,
    MusicVideo,
    Person,
    Photo,
    PhotoAlbum,
    Playlist,
    PlaylistsFolder,
    Program,
    Recording,
    Season,
    Series,
    Studio,
    Trailer,
    TvChannel,
    TvProgram,
    UserRootFolder,
    UserView,
    Video,
    Year,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaSource {
    pub media_streams: Vec<MediaStream>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaStream {
    #[serde(rename = "Type")]
    pub stream_type: MediaStreamType,
    pub display_title: String,
    pub index: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub enum MediaStreamType {
    Video,
    Audio,
    Subtitle,
}

impl TryFrom<String> for MediaStreamType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Video" => Ok(Self::Video),
            "Audio" => Ok(Self::Audio),
            "Subtitle" => Ok(Self::Subtitle),
            _ => Err("Unknown MediaStreamType".into()),
        }
    }
}

impl ApiClient {
    pub async fn get_item(&self, item_id: &str) -> Result<GetItemRes> {
        let url = self
            .root
            .join(&format!("Users/{}/Items/{item_id}", self.account.id))?;

        let res = self.client.get(url).send().await?.json().await?;

        Ok(res)
    }
}
