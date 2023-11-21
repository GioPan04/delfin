use std::sync::Arc;

use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use uuid::Uuid;

use crate::jellyfin_api::{
    api::{latest::GetNextUpOptionsBuilder, shows::GetEpisodesOptionsBuilder},
    api_client::ApiClient,
};

// Gets the next playable media for the given media item.
// For episodes and movies, this just returns the passed in media.
// For TV shows, this looks for the first episode that the user can resume, continue the series
// with, or start from the beginning.
pub async fn get_next_playable_media(
    api_client: Arc<ApiClient>,
    media: BaseItemDto,
) -> Option<BaseItemDto> {
    let media_id = media.id.expect("Media missing id: {media:#?}");
    let media_type = media.type_.expect("Media missing type: {media:#?}");

    match media_type {
        BaseItemKind::Series => get_next_episode(api_client, media_id).await,
        _ => Some(media),
    }
}

pub async fn get_next_episode(api_client: Arc<ApiClient>, media_id: Uuid) -> Option<BaseItemDto> {
    if let Some(resume) = api_client
        .get_continue_watching(
            GetNextUpOptionsBuilder::default()
                .series_id(media_id)
                .limit(1)
                .build()
                .unwrap(),
        )
        .await
        .as_ref()
        .ok()
        .and_then(|resume| resume.first())
    {
        return Some(resume.to_owned());
    };

    if let Some(next_up) = api_client
        .get_next_up(
            GetNextUpOptionsBuilder::default()
                .series_id(media_id)
                .limit(1)
                .build()
                .unwrap(),
        )
        .await
        .ok()
        .as_ref()
        .and_then(|next_up| next_up.first())
    {
        return Some(next_up.to_owned());
    }

    if let Some(items) = api_client
        .get_episodes(
            &GetEpisodesOptionsBuilder::default()
                .series_id(media_id)
                .build()
                .unwrap(),
        )
        .await
        .ok()
        .as_ref()
        .and_then(|episodes| episodes.first())
    {
        return Some(items.to_owned());
    }

    None
}
