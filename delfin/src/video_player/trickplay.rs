use std::{io::Cursor, sync::Arc};

use image::{EncodableLayout, ImageFormat};
use jellyfin_api::types::BaseItemDto;
use relm4::ComponentSender;
use tracing::warn;
use uuid::Uuid;

use crate::{globals::CONFIG, jellyfin_api::api_client::ApiClient, utils::bif::Thumbnail};

use super::{VideoPlayer, VideoPlayerCommandOutput};

pub(crate) fn fetch_trickplay(
    api_client: &Arc<ApiClient>,
    sender: &ComponentSender<VideoPlayer>,
    item: &BaseItemDto,
) {
    let Some(item_id) = item.id else {
        return;
    };

    sender.oneshot_command({
        let api_client = api_client.clone();
        let item = item.clone();
        async move {
            if let Some(thumbnails) = load_native_trickplay(&api_client, &item_id, item).await {
                return VideoPlayerCommandOutput::LoadedTrickplay(Some(thumbnails));
            }

            // If native trickplay wasn't found, fall back on Jellyscrub
            if let Some(thumbnails) = load_jellyscrub_trickplay(&api_client, &item_id).await {
                return VideoPlayerCommandOutput::LoadedTrickplay(Some(thumbnails));
            }

            VideoPlayerCommandOutput::LoadedTrickplay(None)
        }
    });
}

async fn load_native_trickplay(
    api_client: &Arc<ApiClient>,
    item_id: &Uuid,
    item: BaseItemDto,
) -> Option<Vec<Thumbnail>> {
    let (width, trickplay) = item
        .trickplay
        .as_ref()
        .and_then(|trickplay| {
            trickplay
                .get(&item_id.simple().to_string())
                .map(ToOwned::to_owned)
        })
        // Use the first available width width
        .and_then(|trickplay| trickplay.into_iter().next())?;
    let width: usize = width.parse().unwrap();

    let (tile_width, tile_height, thumbnail_count, height, interval) = (
        trickplay.tile_width? as usize,
        trickplay.tile_height? as usize,
        trickplay.thumbnail_count? as usize,
        trickplay.height? as usize,
        trickplay.interval? as usize,
    );

    // TODO: parallelize or fetch tiles as needed
    let mut tiles = vec![];
    for i in 0..=(thumbnail_count / tile_width / tile_height) {
        match api_client.get_trickplay_tile(item_id, width, i).await {
            Ok(tile) => {
                tiles.push(tile);
            }
            Err(err) => {
                warn!("Failed to fetch trickplay tile: {err:#?}");
                return None;
            }
        }
    }

    // TODO: tile processing (loading images and splitting) is very slow

    let tiles: Vec<_> = tiles
        .into_iter()
        .map(|tile| {
            image::load_from_memory_with_format(tile.as_bytes(), ImageFormat::Jpeg).unwrap()
        })
        .collect();

    let thumbnails: Vec<_> = (0..thumbnail_count)
        .map(|thumbnail_index| {
            let tile_index = thumbnail_index / tile_width / tile_height;
            // Get thumbnail position on it's tile
            let (x, y) = {
                let index_in_tile = thumbnail_index % (tile_width * tile_height);
                (index_in_tile % tile_width, index_in_tile / tile_height)
            };
            let thumbnail = tiles[tile_index].crop_imm(
                (x * width) as u32,
                (y * height) as u32,
                width as u32,
                height as u32,
            );
            let mut image = vec![];
            thumbnail
                .write_to(&mut Cursor::new(&mut image), ImageFormat::Jpeg)
                .unwrap();
            Thumbnail {
                timestamp: thumbnail_index * interval / 1000,
                image: image.into(),
            }
        })
        .collect();

    Some(thumbnails)
}

async fn load_jellyscrub_trickplay(
    api_client: &Arc<ApiClient>,
    item_id: &Uuid,
) -> Option<Vec<Thumbnail>> {
    if !CONFIG.read().video_player.jellyscrub {
        return None;
    }

    let manifest = match api_client.get_trickplay_manifest(item_id).await {
        Ok(Some(manifest)) if !manifest.width_resolutions.is_empty() => manifest,
        Ok(None) | Ok(Some(_)) => {
            return None;
        }
        Err(err) => {
            warn!("Error fetching trickplay manifest: {err}");
            return None;
        }
    };

    let width = manifest.width_resolutions.iter().max()?;

    let thumbnails = match api_client.get_trickplay_thumbnails(item_id, *width).await {
        Ok(Some(thumbnails)) => thumbnails,
        Ok(None) => {
            return None;
        }
        Err(err) => {
            warn!("Error fetching trickplay thumbnails: {err}");
            return None;
        }
    };

    Some(thumbnails)
}
