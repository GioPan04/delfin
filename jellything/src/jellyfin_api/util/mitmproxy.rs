use std::fs;

use reqwest::Certificate;

static MITMPROXY_CERT_PATH: &str = ".mitmproxy/mitmproxy-ca.pem";

pub fn mitmproxy_cert() -> Option<Certificate> {
    let home_dir = match dirs::home_dir() {
        Some(d) => d,
        None => return None,
    };

    let path = home_dir.join(MITMPROXY_CERT_PATH);
    if !path.exists() {
        return None;
    }

    let buf = match fs::read(path) {
        Ok(b) => b,
        _ => return None,
    };
    Certificate::from_pem(&buf).ok()
}
