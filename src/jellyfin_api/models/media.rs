use serde::Deserialize;
use speedate::DateTime;

use crate::{
    jellyfin_api::api::item::{ItemType, MediaSource},
    utils::datetime_serde::deserialize_datetime_opt,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Media {
    pub id: String,
    #[serde(rename = "Type")]
    pub item_type: ItemType,
    pub name: String,
    pub image_tags: ImageTags,
    pub user_data: UserData,
    pub parent_backdrop_item_id: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<String>,
    pub index_number: Option<usize>,
    pub parent_index_number: Option<usize>,
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
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageTags {
    pub primary: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserData {
    pub played: bool,
    pub playback_position_ticks: usize,
    pub played_percentage: Option<f64>,
    pub unplayed_item_count: Option<usize>,
}
