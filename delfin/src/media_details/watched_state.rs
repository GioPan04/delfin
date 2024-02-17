use std::sync::Arc;

use anyhow::{anyhow, Result};
use jellyfin_api::types::{BaseItemDto, UserItemDataDto};

use crate::{jellyfin_api::api_client::ApiClient, tr};

pub(crate) trait Played {
    fn played(&self) -> bool;
}

impl Played for BaseItemDto {
    fn played(&self) -> bool {
        self.user_data
            .as_ref()
            .and_then(|user_data| user_data.played)
            .unwrap_or(false)
    }
}

pub(crate) fn watched_label(watched: bool) -> String {
    if watched {
        tr!("media-details-watched").to_owned()
    } else {
        tr!("media-details-unwatched").to_owned()
    }
}

pub(crate) async fn toggle_watched(
    media: &BaseItemDto,
    api_client: &Arc<ApiClient>,
    watched: bool,
) -> Result<UserItemDataDto> {
    let Some(item_id) = media.id else {
        return Err(anyhow!("Media missing ID"));
    };

    if watched {
        api_client.mark_item_played(item_id).await
    } else {
        api_client.mark_item_unplayed(item_id).await
    }
}
