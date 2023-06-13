use std::sync::{Arc, RwLock};

use reqwest::{header::HeaderMap, Url};

use crate::config::{Account, Config, Server};

use super::util::{auth_header::get_auth_header, mitmproxy::mitmproxy_cert, url::httpify};

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

// TODO: remove this
#[allow(dead_code)]
#[derive(Debug)]
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

        let mut client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers);

        #[cfg(debug_assertions)]
        let mitmproxy_cert = mitmproxy_cert();
        if let Some(mitmproxy_cert) = mitmproxy_cert {
            client = client.add_root_certificate(mitmproxy_cert);
        }

        let client = client.build().unwrap();

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
