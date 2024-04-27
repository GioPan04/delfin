use anyhow::Result;
use bytes::{Buf, Bytes};

// BIF decoder for scrubber thumbnails
// File format documented here: https://developer.roku.com/en-ca/docs/developer-program/media-playback/trick-mode/bif-file-creation.md#file-format
// Should more or less match Jellyscrub's implementation:
// https://github.com/nicknsy/jellyscrub/blob/236ed83c655b1f77fee1f59613fda8572646044b/Nick.Plugin.Jellyscrub/Api/trickplay.js#L315-L366

const BIF_MAGIC: [u8; 8] = [0x89, 0x42, 0x49, 0x46, 0x0D, 0x0A, 0x1A, 0x0A];
const SUPPORTED_BIF_VERSION: i32 = 0;

struct ThumbnailIndex {
    timestamp: i32,
    offset: i32,
}

#[derive(Debug)]
pub struct Thumbnail {
    pub timestamp: usize,
    pub image: Bytes,
}

pub fn decode_bif(bif: &Bytes) -> Result<Vec<Thumbnail>> {
    let (timestamp_multiplier, thumbnail_indices) = {
        // Create a mutable copy of the data in this block for easy reading
        let mut bif = bif.clone();

        if !bif.starts_with(&BIF_MAGIC) {
            anyhow::bail!("BIF magic bytes check failed");
        }
        bif.advance(BIF_MAGIC.len());

        let version = bif.get_i32_le();
        if version != SUPPORTED_BIF_VERSION {
            anyhow::bail!(
                "BIF version is not supported: {version} (expected {SUPPORTED_BIF_VERSION})"
            );
        }

        let num_images = bif.get_i32_le();

        let timestamp_multiplier = match bif.get_i32_le() {
            0 => 1000,
            n => n,
        };

        // Skip reserved bytes
        bif.advance(44);

        let mut thumbnail_indices = Vec::default();

        for _ in 0..num_images {
            thumbnail_indices.push(ThumbnailIndex {
                timestamp: bif.get_i32_le(),
                offset: bif.get_i32_le(),
            });
        }

        (timestamp_multiplier, thumbnail_indices)
    };

    let mut thumbnails = Vec::default();

    for (i, thumbnail) in thumbnail_indices.iter().enumerate() {
        let next_offset = if (i + 1) >= thumbnail_indices.len() {
            bif.len()
        } else {
            thumbnail_indices[i + 1].offset as usize
        };

        let image = bif.slice((thumbnail.offset as usize)..next_offset);

        thumbnails.push(Thumbnail {
            timestamp: (thumbnail.timestamp * timestamp_multiplier / 1000) as usize,
            image,
        });
    }

    Ok(thumbnails)
}
