use anyhow::Result;
use jellyfin_api::types::BaseItemDto;
use std::collections::VecDeque;

use crate::jellyfin_api::{
    api_client::ApiClient,
    image_cache::{ImageId, ImageKind, ImageParams, ImageUrl},
};

impl ApiClient {
    /// This method can be used with any [`ImageUrl`] returned by the helper methods.
    ///
    /// In general, the requested image should exist because it was promised by the Jellyfin API.
    /// However, it is possible for the API to return an empty image (HTTP status 200 with 0 bytes).
    ///
    /// This method will either succeed or return an error, but will not return an empty image.
    /// TODO: Should we also make sure the API returned *valid* image bytes before caching them?
    pub async fn get_image(&self, image: &ImageUrl) -> Result<VecDeque<u8>> {
        self.cache.get(self, image).await
    }

    pub fn get_episode_primary_image_url(
        &self,
        item: &BaseItemDto,
        height: u16,
    ) -> Option<ImageUrl> {
        Some(self.cache.image_url(
            self,
            &ImageId::from_item(item, ImageKind::Primary)?,
            &ImageParams::fill_height(height),
        ))
    }

    pub fn get_episode_thumbnail_or_backdrop_url(
        &self,
        item: &BaseItemDto,
        height: u16,
    ) -> Option<ImageUrl> {
        Some(self.cache.image_url(
            self,
            &ImageId::from_item(item, ImageKind::Thumb).or(ImageId::from_item_backdrop(item))?,
            &ImageParams::fill_height(height),
        ))
    }

    pub fn get_parent_or_item_thumbnail_url(
        &self,
        item: &BaseItemDto,
        height: u16,
    ) -> Option<ImageUrl> {
        let id = ImageId::from_item_parent_thumb(item)
            .or(ImageId::from_item_parent_backdrop(item))
            .or(ImageId::from_item(item, ImageKind::Primary))?;

        Some(
            self.cache
                .image_url(self, &id, &ImageParams::fill_height(height)),
        )
    }

    pub fn get_parent_or_item_primary_image_url(&self, item: &BaseItemDto) -> Option<ImageUrl> {
        let id = ImageId::from_item_parent_backdrop(item)
            .or(ImageId::from_item(item, ImageKind::Primary))?;

        Some(
            self.cache
                .image_url(self, &id, &ImageParams::fill_width(200)),
        )
    }

    pub fn get_collection_thumbnail_url(&self, item: &BaseItemDto) -> Option<ImageUrl> {
        // TODO: is this the same logic as get_parent_or_item_primary_image_url? Just different size/quality?
        // Then why if the fuinction named "thumbnail" ???
        let id = ImageId::from_item_parent_backdrop(item)
            .or(ImageId::from_item(item, ImageKind::Primary))?;

        Some(
            self.cache
                .image_url(self, &id, &ImageParams::fill_width(700).quality(100)),
        )
    }

    pub fn get_parent_or_item_backdrop_url(&self, item: &BaseItemDto) -> Option<ImageUrl> {
        let id = ImageId::from_item_parent_backdrop(item)
            .or(ImageId::from_item(item, ImageKind::Backdrop))?;

        Some(
            self.cache
                .image_url(self, &id, &ImageParams::fill_width(350)),
        )
    }

    pub fn get_backdrop_url(&self, item: &BaseItemDto) -> Option<ImageUrl> {
        Some(self.cache.image_url(
            self,
            &ImageId::from_item(item, ImageKind::Backdrop)?,
            &ImageParams::fill_width(1440).quality(80),
        ))
    }

    pub fn get_next_up_thumbnail_url(&self, item: &BaseItemDto) -> Option<ImageUrl> {
        Some(self.cache.image_url(
            self,
            &ImageId::from_item(item, ImageKind::Primary)?,
            &ImageParams::fill_height(150),
        ))
    }
}
