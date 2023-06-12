use reqwest::Client;

use super::{api_client::APP_USER_AGENT, mitmproxy::mitmproxy_cert};

pub fn get_unauthed_client() -> Client {
    let mut client = Client::builder().user_agent(APP_USER_AGENT);

    #[cfg(debug_assertions)]
    let mitmproxy_cert = mitmproxy_cert();
    if let Some(mitmproxy_cert) = mitmproxy_cert {
        client = client.add_root_certificate(mitmproxy_cert);
    }

    client.build().unwrap()
}
