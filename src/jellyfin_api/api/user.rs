use std::collections::VecDeque;

use anyhow::Result;
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};

use crate::jellyfin_api::{
    unauthed_client::get_unauthed_client,
    util::{auth_header::get_auth_header, url::httpify},
};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct AuthenticateByNameReqBody {
    username: String,
    #[serde(rename = "Pw")]
    password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateByNameRes {
    pub user: AuthenticateByNameResUser,
    pub access_token: String,
    pub server_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateByNameResUser {
    pub id: String,
    pub name: String,
}

pub async fn authenticate_by_name(
    url: &str,
    device_id: &str,
    username: &str,
    password: &str,
) -> Result<AuthenticateByNameRes> {
    let client = get_unauthed_client();

    let url = httpify(url);
    let url = format!("{}Users/AuthenticateByName", url);

    let res = client
        .post(url)
        .header("authorization", get_auth_header(device_id, None))
        .json(&AuthenticateByNameReqBody {
            username: username.into(),
            password: password.into(),
        })
        .send()
        .await?;

    match res.status() {
        StatusCode::OK => {
            let res = res.json().await?;
            Ok(res)
        }
        StatusCode::UNAUTHORIZED => anyhow::bail!("Wrong username or password."),
        _ => {
            println!("Sign in error: {:#?}", res);
            anyhow::bail!("Error signing in.");
        }
    }
}

pub async fn get_user_avatar(url: &str, user_id: &str) -> Result<VecDeque<u8>> {
    let client = get_unauthed_client();

    let url = httpify(url);
    let url = Url::parse(&url)?.join(&format!("Users/{user_id}/Images/Primary?width=40"))?;

    Ok(client
        .get(url)
        .send()
        .await?
        .bytes()
        .await?
        .into_iter()
        .collect())
}
