use uuid::Uuid;

const CLIENT_NAME: &str = "Delfin";

pub fn get_auth_header(device_id: &Uuid, access_token: Option<&str>) -> String {
    let device = whoami::devicename();
    let version = env!("CARGO_PKG_VERSION");

    let mut auth = format!(
        r#"MediaBrowser Client="{}", Device="{}", DeviceId="{}", Version="{}""#,
        CLIENT_NAME, device, device_id, version
    );

    if let Some(auth_token) = access_token {
        auth.push_str(&format!(r#", Token="{}""#, auth_token));
    }

    auth
}
