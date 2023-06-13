use reqwest::Url;

use crate::{config, jellyfin_api::util::url::httpify};

pub fn get_stream_url(server: &config::Server, item_id: &str) -> String {
    Url::parse(&httpify(&server.url))
        .unwrap()
        .join(&format!("Videos/{}/stream?Static=true", item_id))
        .unwrap()
        .to_string()
}
