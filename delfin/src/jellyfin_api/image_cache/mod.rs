use crate::jellyfin_api::api_client::ApiClient;
use anyhow::{bail, Result};
use reqwest::StatusCode;
use std::collections::VecDeque;
use std::path::PathBuf;
use tracing::error;

mod image;
pub use image::{ImageId, ImageKind};
mod image_params;
pub use image_params::{ImageParams, ImageSize};
mod image_url;
pub use image_url::ImageUrl;

/// A local disk-backed image cache
///
/// Appends `delfin/{API_INSTANCE}` to `$XDG_CACHE_DIR`, and will panic if either:
///
/// - both `$XDG_CACHE_DIR` and `$HOME` environment variables are empty
/// - or creating the cache directory failed
#[derive(Clone, Debug)]
pub struct ImageCache {
    basedir: PathBuf,
}

impl ImageCache {
    pub fn new(api_instance: &str) -> Self {
        let basedir = if let Some(dir) = dirs::cache_dir() {
            dir.join("delfin").join(api_instance)
        } else {
            error!("Failed to find $XDG_CACHE_DIR or $HOME. Exiting the program now.");
            panic!();
        };

        if !basedir.is_dir() {
            if let Err(e) = std::fs::create_dir_all(&basedir) {
                error!(
                    "Failed to create image cache {}: {}\nExiting the program now.",
                    basedir.display(),
                    e
                );
                panic!();
            }
        }

        Self { basedir }
    }

    /// Gets a specific [`ImageUrl`] from local cache or remote API.
    ///
    /// The image should exist on the server as promised by the API when
    /// building the [`ImageUrl`], so this function returns a Result.
    ///
    /// However, when the remote image is 0 bytes it is considered an error.
    pub async fn get(&self, api_client: &ApiClient, image_url: &ImageUrl) -> Result<VecDeque<u8>> {
        if let Some(bytes) = self.get_local(image_url).await? {
            Ok(bytes.into_iter().collect())
        } else {
            let bytes = self.get_remote(api_client, image_url).await?;
            // We don't care if saving doesn't succeed, we still have all the bytes we need
            self.save_local(image_url, &bytes).await;
            Ok(bytes.into_iter().collect())
        }
    }

    /// Build an [`ImageUrl`] for a specific [`ImageId`] with some extra [`ImageParams`].
    ///
    /// This operation cannot fail, and guarantees the image exists on the server, because the
    /// [`ImageId`] was assembled from an API response.
    ///
    /// You can then use [`ImageCache::get`] to read the actual content of the produced [`ImageUrl`],
    /// whether it's returned from the local disk or from the remote network.
    pub fn image_url(
        &self,
        api_client: &ApiClient,
        id: &ImageId,
        params: &ImageParams,
    ) -> ImageUrl {
        let mut url = api_client
            .root
            .join(&format!("Items/{}/Images/{}", id.uuid, id.kind))
            .unwrap();

        {
            let mut query = url.query_pairs_mut();
            let (k, v) = params.size.key_val();
            query.append_pair(k, &v.to_string());
            query.append_pair("quality", &params.quality.to_string());
            query.append_pair("tag", &id.tag);
        }

        ImageUrl::new(url, id.clone(), params.clone())
    }

    /// Fetch an [`ImageUrl`] from the local cache.
    ///
    /// Return Ok(Some(bytes)) when successful,
    /// Ok(None) when the server has no corresponding image,
    /// and Err(e) when an error occurred during fetch.
    async fn get_local(&self, image_url: &ImageUrl) -> Result<Option<Vec<u8>>> {
        let filename = self.basedir.join(image_url.as_path());

        if tokio::fs::try_exists(&filename).await? {
            Ok(Some(tokio::fs::read(&filename).await?))
        } else {
            Ok(None)
        }
    }

    /// Save some bytes to disk for caching.
    ///
    /// Errors are discarded but logged, as they will not affect behavior of the program.
    async fn save_local(&self, image_url: &ImageUrl, bytes: &[u8]) {
        let filename = self.basedir.join(image_url.as_path());

        if let Err(e) = tokio::fs::write(&filename, bytes).await {
            error!("Failed to write cache file {}", filename.display());
            error!("{}", e);
        }
    }

    /// Fetch an [`ImageUrl`] from the remote API.
    ///
    /// The image should exist as promised by the API when building the [`ImageUrl`].
    /// This method returns Ok(bytes) or Err(e) when fetching failed.
    ///
    /// An empty image (0 bytes) with 200 status code is considered an error.
    async fn get_remote(&self, api_client: &ApiClient, image_url: &ImageUrl) -> Result<Vec<u8>> {
        let response = api_client.client.get(image_url.url.clone()).send().await?;
        let img_bytes: Vec<u8> = match response.status() {
            StatusCode::OK => {
                // TODO: maybe check mimetype too?
                // TODO: maybe check that's actual valid bytes?
                response.bytes().await?.into_iter().collect()
            }
            StatusCode::NOT_FOUND => {
                unreachable!(
                    "An ImageUrl was built for UUID {} however the API returned 404.",
                    image_url.id.uuid
                );
            }
            _ => {
                bail!(
                    "API Image request for UUID {} failed with HTTP status {} from URL {}",
                    image_url.id.uuid,
                    response.status(),
                    image_url.url
                );
            }
        };

        if img_bytes.is_empty() {
            // Why does Jellyfin return 200 OK with 0 bytes PNG sometimes?
            bail!("Image {} is empty!", image_url.url);
        }

        Ok(img_bytes)
    }
}
