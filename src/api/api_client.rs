use std::sync::{Arc, RwLock};

use reqwest::{header::HeaderMap, Url};

use crate::config::{Account, Config, Server};

use super::{auth_header::get_auth_header, url::httpify};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

// TODO: remove this
#[allow(dead_code)]
pub struct ApiClient {
    config: Arc<RwLock<Config>>,
    server: Server,
    pub account: Account,
    pub client: reqwest::Client,
    pub root: Url,
}

impl ApiClient {
    pub fn new(config: Arc<RwLock<Config>>, server: &Server, account: &Account) -> ApiClient {
        let device_id = config.read().unwrap().device_id.clone();

        let auth_header = get_auth_header(&device_id, Some(&account.access_token));

        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, auth_header.parse().unwrap());

        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()
            .unwrap();

        let root = Url::parse(&httpify(&server.url)).unwrap();

        ApiClient {
            config,
            server: server.clone(),
            account: account.clone(),
            client,
            root,
        }
    }
}
