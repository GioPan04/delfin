use std::{collections::VecDeque, thread, time::Duration};

use anyhow::Result;
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

use crate::jellyfin_api::{
    api_client::ApiClient,
    unauthed_client::get_unauthed_client,
    util::{auth_header::get_auth_header, url::httpify},
};
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct QuickConnectProgressRes {
    pub secret: String,
    pub code: String,
    // #[serde(deserialize_with = "deserialize_bool")]
    authenticated: bool,
}
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct AuthenticateByQuickConnectReqBody {
    secret: String,
}
// #[derive(Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct AuthenticateByQuickConnectRes {

// }
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
    pub server_id: Uuid,
    pub session_info: AuthenticateByNameResSessionInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateByNameResUser {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticateByNameResSessionInfo {
    pub device_id: Uuid,
}

pub async fn authenticate_by_name(
    url: &str,
    device_id: &Uuid,
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
            error!("Sign in error: {:#?}", res);
            anyhow::bail!("Error signing in.");
        }
    }
}
pub async fn authenticate_pin_init(url: &str, device_id: &Uuid) -> Result<QuickConnectProgressRes> {
    let client = get_unauthed_client();
    let initiate_url: String = httpify(url);
    let initiate_url = format!("{}QuickConnect/Initiate", url);

    Ok(client.get(initiate_url).send().await?.json().await?)

}
pub async fn authenticate_by_pin(url: &str, device_id: &Uuid, secret: &str) -> Result<AuthenticateByNameRes> {
    let client = get_unauthed_client();
    let connect_url = httpify(url);
    let connect_url = format!("{}/QuickConnect/Connect?Secret={}", url, secret);
    let mut res: QuickConnectProgressRes = client.get(connect_url.clone()).send().await?.json().await?;

    while !res.authenticated {
        thread::sleep(Duration::from_millis(250)); // change this to use callbacks instead of pin
        res = client.get(connect_url.clone()).send().await?.json().await?;
    }
    let authenticate_url = httpify(url);
    let authenticate_url = format!("{}/Users/AuthenticateWithQuickConnect", url);
    let auth_res = client.post(authenticate_url).header("authorization", get_auth_header(device_id, None)).json(&AuthenticateByQuickConnectReqBody {
        secret: res.secret
    }).send().await?.json().await?;

    Ok(auth_res)
}
pub async fn get_user_avatar(url: &str, user_id: &Uuid) -> Result<VecDeque<u8>> {
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

impl ApiClient {
    pub async fn sign_out(&self) -> Result<()> {
        let url = self.root.join("/Sessions/Logout")?;
        self.client.post(url).send().await?;
        Ok(())
    }
}
