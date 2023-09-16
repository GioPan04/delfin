use anyhow::Result;
use reqwest::Url;
use serde::Deserialize;

use crate::{
    config,
    jellyfin_api::{api_client::ApiClient, models::media::Media, util::url::httpify},
};

pub fn get_stream_url(server: &config::Server, item_id: &str) -> String {
    Url::parse(&httpify(&server.url))
        .unwrap()
        .join(&format!("Videos/{}/stream?static=true", item_id))
        .unwrap()
        .to_string()
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
    pub async fn get_item(&self, item_id: &str) -> Result<Media> {
        let url = self
            .root
            .join(&format!("Users/{}/Items/{item_id}", self.account.id))?;

        let mut res: Media = self.client.get(url).send().await?.json().await?;

        self.backdrop_image_tags_to_urls(&mut res)?;

        Ok(res)
    }

    fn backdrop_image_tags_to_urls(&self, item: &mut Media) -> Result<()> {
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
