use anyhow::Result;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use super::{auth_header::get_auth_header, url::httpify};

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
    let url = httpify(url);
    let url = format!("{}/Users/AuthenticateByName", url);

    let res = Client::new()
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
