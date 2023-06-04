const CLIENT_NAME: &str = "Jellything";

pub fn get_auth_header(device_id: &str, auth_token: Option<&str>) -> String {
    let device = whoami::devicename();
    let version = env!("CARGO_PKG_VERSION");

    let mut auth = format!(
        r#"MediaBrowser Client="{}", Device="{}", DeviceId="{}", Version="{}""#,
        CLIENT_NAME, device, device_id, version
    );

    if let Some(auth_token) = auth_token {
        auth.push_str(&format!(r#", Token="{}""#, auth_token));
    }

    auth
}
