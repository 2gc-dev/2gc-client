use crate::process::get_free_port;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    static ref SSHSETTINGS: Arc<Mutex<Vec<ConnectSSHParams>>> = Arc::new(Mutex::new(Vec::new()));
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectSSHParams {
    pub tunnel_id: String,
    pub localport: u16,
}

impl ConnectSSHParams {
    pub fn get_ssh_params(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        let port_data = format!("-P {}", self.localport);
        args.push(port_data);
        args
    }
    pub async fn new(tunnel_id: String) -> ConnectSSHParams {
        let serve_key: String = tunnel_id.chars().take(8).collect();
        let localport = get_free_port(&serve_key).await;
        ConnectSSHParams {
            localport: localport,
            tunnel_id: tunnel_id,
        }
    }
}

pub async fn get_or_create_ssh_params(tunnel_id: &str) -> ConnectSSHParams {
    let mut settings = SSHSETTINGS.lock().await;
    if let Some(p) = settings.iter().find(|params| params.tunnel_id == tunnel_id) {
        p.clone()
    } else {
        let server_params = ConnectSSHParams::new(tunnel_id.to_string()).await;
        let local = server_params.clone();
        settings.push(local);
        server_params
    }
}
