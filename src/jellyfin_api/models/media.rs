use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Media {
    pub id: String,
    pub name: String,
    pub image_tags: ImageTags,
    pub user_data: UserData,
    pub parent_backdrop_item_id: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<String>,
    pub index_number: Option<usize>,
    pub parent_index_number: Option<usize>,
    pub overview: Option<String>,
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
