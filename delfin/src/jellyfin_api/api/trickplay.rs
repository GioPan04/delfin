use std::format;

use anyhow::Result;
use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    jellyfin_api::api_client::ApiClient,
    utils::bif::{decode_bif, Thumbnail},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrickplayManifest {
    pub width_resolutions: Vec<usize>,
}

impl ApiClient {
    pub async fn get_trickplay_manifest(&self, id: &Uuid) -> Result<Option<TrickplayManifest>> {
        let url = self.root.join(&format!("Trickplay/{id}/GetManifest"))?;

        let res = self.client.get(url).send().await?;
        if res.status() == StatusCode::NOT_FOUND {
            // Returns a 404 if item doesn't have a trickplay manifest
            return Ok(None);
        }

        Ok(Some(res.json().await?))
    }

    pub async fn get_trickplay_thumbnails(
        &self,
        id: &Uuid,
        width: usize,
    ) -> Result<Option<Vec<Thumbnail>>> {
        let url = self.root.join(&format!("Trickplay/{id}/{width}/GetBIF"))?;

        let res = self.client.get(url).send().await?;
        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let bif = res.bytes().await?;
        let thumbnails = decode_bif(&bif)?;

        Ok(Some(thumbnails))
    }
}
