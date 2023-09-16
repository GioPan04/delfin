use anyhow::Result;
use reqwest::Url;
use serde::Deserialize;
use speedate::DateTime;

use crate::{
    config,
    jellyfin_api::{api_client::ApiClient, models::media::UserData, util::url::httpify},
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
    pub id: String,
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
    #[serde(rename = "BackdropImageTags")]
    pub backdrop_image_urls: Option<Vec<String>>,
    pub parent_index_number: Option<usize>,
    pub index_number: Option<usize>,
    pub user_data: Option<UserData>,
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

        let mut res: GetItemRes = self.client.get(url).send().await?.json().await?;

        self.backdrop_image_tags_to_urls(&mut res)?;

        Ok(res)
    }

    fn backdrop_image_tags_to_urls(&self, item: &mut GetItemRes) -> Result<()> {
        let item_id = &item.id;

        for backdrop in item.backdrop_image_urls.iter_mut().flatten() {
            let mut url = self
                .root
                .join(&format!("Items/{item_id}/Images/Backdrop"))?;

            url.query_pairs_mut()
                .append_pair("tag", backdrop)
                .append_pair("maxWidth", "1440")
                .append_pair("quality", "80");

            *backdrop = url.to_string();
        }

        Ok(())
    }
}
