use crate::process::get_free_port;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    static ref RDPSETTINGS: Arc<Mutex<Vec<ConnectRDPParams>>> = Arc::new(Mutex::new(Vec::new()));
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScreenSize {
    w: u16,
    h: u16,
}

impl Default for ScreenSize {
    fn default() -> Self {
        ScreenSize { w: 0, h: 0 }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectRDPParams {
    pub is_sound: bool,
    pub is_drives: bool,
    pub is_printers: bool,
    pub is_wallpaper: bool,
    pub is_max_screen: bool,
    pub screen_size: ScreenSize,
    pub tunnel_id: String,
    pub localport: u16,
}

impl ConnectRDPParams {
    pub fn create_default(tunnel_id: &str, localport: u16) -> Self {
        ConnectRDPParams {
            is_sound: true,
            is_drives: true,
            is_printers: true,
            is_wallpaper: true,
            is_max_screen: true,
            screen_size: ScreenSize::default(),
            tunnel_id: tunnel_id.to_string(),
            localport,
        }
    }
    pub fn update_default(
        &mut self,
        is_sound: bool,
        is_drives: bool,
        is_printers: bool,
        is_wallpaper: bool,
        is_max_screen: bool,
        w: u16,
        h: u16,
    ) {
        self.is_sound = is_sound;
        self.is_drives = is_drives;
        self.is_printers = is_printers;
        self.is_wallpaper = is_wallpaper;
        self.is_max_screen = is_max_screen;
        self.screen_size = ScreenSize { w, h };
    }

    pub fn get_rdp_command(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        // Проверка на максимальный размер экрана
        if self.is_max_screen {
            args.push("/max".to_string());
        } else {
            // Указание ширины и высоты экрана
            args.push(format!("/w:{}", self.screen_size.w));
            args.push(format!("/h:{}", self.screen_size.h));
        }

        // Добавление других параметров, если они активны
        if self.is_sound {
            args.push("/sound".to_string());
        } else {
            args.push("/nosound".to_string());
        }

        if self.is_drives {
            args.push("/drives".to_string());
        } else {
            args.push("/nodrives".to_string());
        }

        if self.is_printers {
            args.push("/printers".to_string());
        } else {
            args.push("/[no]printers".to_string());
        }

        if self.is_wallpaper {
            args.push("/wallpaper".to_string());
        } else {
            args.push("/nowallpaper".to_string());
        }

        args
    }

    pub fn new(
        is_sound: bool,
        is_drives: bool,
        is_printers: bool,
        is_wallpaper: bool,
        is_max_screen: bool,
        screen_size: ScreenSize,
        tunnel_id: String,
        localport: u16,
    ) -> Self {
        ConnectRDPParams {
            is_sound,
            is_drives,
            is_printers,
            is_wallpaper,
            is_max_screen,
            screen_size,
            tunnel_id,
            localport,
        }
    }
}

pub async fn _get_connect_params(tunnel_id: &str) -> Option<ConnectRDPParams> {
    let settings = RDPSETTINGS.lock().await;
    settings
        .iter()
        .find(|params| params.tunnel_id == tunnel_id)
        .cloned()
}

pub async fn _save_connect_params(connect_params: &ConnectRDPParams) {
    let params = connect_params.clone();
    let mut settings = RDPSETTINGS.lock().await;

    if let Some(existing_params) = settings
        .iter_mut()
        .find(|p| p.tunnel_id == params.tunnel_id)
    {
        *existing_params = params;
    } else {
        settings.push(params);
    }
}

pub async fn get_or_create_rdp_params(tunnel_id: &str) -> ConnectRDPParams {
    let mut settings = RDPSETTINGS.lock().await;
    if let Some(p) = settings.iter().find(|params| params.tunnel_id == tunnel_id) {
        p.clone()
    } else {
        let key_data = tunnel_id.to_string();
        let serve_key:String = key_data.chars().take(8).collect();
        let localport = get_free_port(&serve_key).await;
        //let localport = 1155;
        let server_params = ConnectRDPParams::create_default(tunnel_id, localport);
        let local = server_params.clone();
        settings.push(local);
        server_params
    }
}

pub async fn update_connect_params(
    tunnel_id: &str,
    is_sound: bool,
    is_drives: bool,
    is_printers: bool,
    is_wallpaper: bool,
    is_max_screen: bool,
    w: u16,
    h: u16,
) {
    let mut settings = RDPSETTINGS.lock().await;

    if let Some(server_process) = settings
        .iter_mut()
        .find(|params| params.tunnel_id == tunnel_id)
    {
        server_process.update_default(
            is_sound,
            is_drives,
            is_printers,
            is_wallpaper,
            is_max_screen,
            w,
            h,
        );
    }
}
