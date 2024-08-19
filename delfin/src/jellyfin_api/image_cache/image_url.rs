use crate::jellyfin_api::image_cache::{ImageId, ImageParams};
use reqwest::Url;
use std::path::PathBuf;

/// A complete URL to a backend image, with sizing parameters and cache invalidation tag.
///
/// The image is guaranteed to exist, according to the Jellyfin API,
/// because this struct was assembled from data returned by the API using
/// an [`ApiClient`] helper method.
///
/// Use [`ApiClient::get_image`] to actually get it from cache or from the remote
/// Jellyfin server.
#[derive(Clone, Debug)]
pub struct ImageUrl {
    pub url: Url,
    pub id: ImageId,
    pub params: ImageParams,
}

impl ImageUrl {
    pub fn new(url: Url, id: ImageId, params: ImageParams) -> Self {
        Self { url, id, params }
    }

    /// Produce a unique filename for the image URL.
    ///
    /// The produced filename is only as unique as UUIDs returned by the Jellyfin API, which might
    /// collide with other Jellyfin instances. However, as the image cache is unique to an instance
    /// there should not be any collision happening.
    pub fn as_path(&self) -> PathBuf {
        format!("{}-{}{}", self.id.uuid, self.id.kind, self.params)
            .try_into()
            .unwrap()
    }
}
