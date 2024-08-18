use crate::{config::Server as ConfigServer, servers::server_list_item::ServerListItem};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::{Duration, Instant};
use tracing::{error, info};
use uuid::Uuid;

const JELLYFIN_DISCO_ADDR: &SocketAddrV4 =
    &SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), 7359);

pub const JELLYFIN_REQUEST: &[u8] = b"Who is JellyfinServer?";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JellyfinResponse {
    pub address: String,
    pub id: Uuid,
    pub name: String,
    pub endpoint_address: Option<String>,
}

impl From<JellyfinResponse> for ServerListItem {
    fn from(r: JellyfinResponse) -> ServerListItem {
        ServerListItem {
            url: r.address,
            name: r.name,
        }
    }
}

impl From<JellyfinResponse> for ConfigServer {
    fn from(r: JellyfinResponse) -> ConfigServer {
        ConfigServer {
            id: r.id,
            url: r.address,
            name: r.name,
            accounts: vec![],
        }
    }
}

/// Queries LAN address 255.255.255.255:7359 for jellyfin servers
///
/// Fails after timeout, returns an empty list if no server was found.
pub fn lan_discovery(timeout: Duration) -> Vec<JellyfinResponse> {
    let mut res: Vec<JellyfinResponse> = vec![];
    let now = Instant::now();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let _ = socket.set_read_timeout(Some(timeout));
    let _ = socket.set_write_timeout(Some(timeout));
    let _ = socket.set_broadcast(true);

    // Send magic Jellyfin query
    if let Err(e) = socket.send_to(JELLYFIN_REQUEST, JELLYFIN_DISCO_ADDR) {
        error!("Could not send Jellyfin LAN discovery query: {e}");
        return res;
    }

    let mut buf = [0; 4096];
    while now.elapsed() < timeout {
        if let Ok(size) = socket.recv(&mut buf) {
            if let Ok(response) = serde_json::from_slice::<JellyfinResponse>(&buf[0..size]) {
                info!(
                    "Received LAN advertisement for Jellyfin server {}",
                    response.address
                );
                res.push(response);
            }
        } else {
            // Timeout
            return res;
        }
    }

    res
}
