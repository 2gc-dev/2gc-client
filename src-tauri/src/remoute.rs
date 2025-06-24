use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use os_info;

const USER_AGENT: &str = "2GC-CloudBridge/1.4.3";
const KEYRING_SERVICE: &str = "2gc-cloudbridge";
const KEYRING_USER: &str = "refresh_token";

// src-tauri/src/remoute.rs

#[cfg(any(target_os = "windows", target_os = "macos"))]
use hostname::get;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind, RefreshKind};

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[derive(Serialize)]
struct SystemInfo {
    os: String,
    cpu: String,
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    hostname: String,
}

#[derive(Debug, Serialize)]
struct ClientInfo {
    version: String,
    os: String,
    arch: String,
    cpu_info: String,
    memory_total: u64,
    memory_used: u64,
    hostname: String,
    timezone: String,
    locale: String,
    screen_resolution: String,
    display_count: u32,
    network_interfaces: Vec<String>,
}

fn get_client_info() -> ClientInfo {
    let os_info = os_info::get();
    let sys = System::new_all();
    
    // Получаем информацию о CPU
    let cpu_info = sys.cpus()
        .get(0)
        .map(|cpu| format!("{} {} cores", cpu.brand(), sys.cpus().len()))
        .unwrap_or_else(|| "Unknown".to_string());

    // Получаем информацию о памяти
    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();

    // Получаем имя хоста
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    // Получаем временную зону
    let timezone = chrono::Local::now().format("%z").to_string();

    // Получаем локаль
    let locale = sysinfo::System::long_os_version()
        .unwrap_or_else(|| "Unknown".to_string());

    // Получаем информацию о сетевых интерфейсах
    let network_interfaces = get_if_addrs::get_if_addrs()
        .map(|interfaces| {
            interfaces
                .iter()
                .filter(|iface| !iface.is_loopback())
                .map(|iface| format!("{}: {:?}", iface.name, iface.addr))
                .collect()
        })
        .unwrap_or_default();

    ClientInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        os: os_info.os_type().to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu_info,
        memory_total,
        memory_used,
        hostname,
        timezone,
        locale,
        screen_resolution: "Unknown".to_string(), // Будет заполнено на фронтенде
        display_count: 0, // Будет заполнено на фронтенде
        network_interfaces,
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub async fn collect_and_send_info_with_token(access_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let sys = System::new_with_specifics(
        RefreshKind::default()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    let cpu = sys
        .cpus()
        .get(0)
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let cpu_usage = sys
        .cpus()
        .get(0)
        .map(|c| c.cpu_usage())
        .unwrap_or(0.0);

    let hostname = get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    let info = SystemInfo {
        os: std::env::consts::OS.to_string(),
        cpu,
        cpu_usage,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        hostname,
    };

    let client = Client::new();
    let client_info = get_client_info();
    let res = client
        .post("https://lk.2gc.ru/api/system-info/")
        .bearer_auth(access_token)
        .header("User-Agent", USER_AGENT)
        .header("X-Client-Type", "desktop")
        .header("X-Client-Version", &client_info.version)
        .header("X-Client-OS", &client_info.os)
        .header("X-Client-Arch", &client_info.arch)
        .header("X-Client-CPU", &client_info.cpu_info)
        .header("X-Client-Memory", &format!("{}/{}", client_info.memory_used, client_info.memory_total))
        .header("X-Client-Hostname", &client_info.hostname)
        .header("X-Client-Timezone", &client_info.timezone)
        .header("X-Client-Locale", &client_info.locale)
        .header("X-Client-Network", &client_info.network_interfaces.join(";"))
        .json(&info)
        .send()
        .await?;

    if res.status().is_success() {
        println!("🔧 Системная информация отправлена успешно");
    } else {
        println!("❌ Ошибка при отправке системной информации: {}", res.status());
    }

    Ok(())
}


lazy_static! {
    static ref USER: Arc<Mutex<Option<User>>> = Arc::new(Mutex::new(None));
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerResponse {
    pub id: String,
    pub name: String,
    pub servers: Vec<Server>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub services: Vec<Service>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub protocol: String,
    pub port: u16,
    pub access_url: String,
}

#[derive(Debug, Deserialize)]
struct Claims {
    exp: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub refresh_token: String,
    pub access_token: String,
    pub exp: u64,
    pub companys: Vec<ServerResponse>,
    pub services: HashMap<String, Service>,
    pub is_save: bool,
    pub name: String,
}

// ---------- KEYRING HELPERS ---------

fn get_refresh_token_from_keyring() -> Option<String> {
    match keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        Ok(entry) => match entry.get_password() {
            Ok(token) => Some(token),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn set_refresh_token_to_keyring(token: &str) {
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        let _ = entry.set_password(token);
    }
}

fn delete_refresh_token_from_keyring() {
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        let _ = entry.delete_password();
    }
}

// -------------------------------------

impl User {
    pub async fn try_remember() -> Result<User, Box<dyn Error + Send + Sync>> {
        let refresh_token = get_refresh_token_from_keyring();

        println!("refresh_token {:?}", refresh_token);

        match refresh_token {
            Some(token) => {
                let url = "https://lk.2gc.ru/api/token/refresh/";
                let login_refresh = serde_json::json!({
                    "refresh": token,
                });

                let client = Client::new();
                let client_info = get_client_info();
                let response = client
                    .post(url)
                    .header("User-Agent", USER_AGENT)
                    .header("X-Client-Type", "desktop")
                    .header("X-Client-Version", &client_info.version)
                    .header("X-Client-OS", &client_info.os)
                    .header("X-Client-Arch", &client_info.arch)
                    .json(&login_refresh)
                    .send()
                    .await?;
                let response_status = response.status();
                let response_tokens: Value = response.json().await?;
                println!("[AUTH] Refresh response: {:?}", response_tokens);
                println!("[AUTH] Refresh status: {}", response_status);

                let access_token = response_tokens["access"]
                    .as_str()
                    .ok_or("Missing access token")?
                    .to_string();
                let refresh_token = response_tokens["refresh"]
                    .as_str()
                    .ok_or("Missing refresh token")?
                    .to_string();

                // Обновляем token в keyring
                set_refresh_token_to_keyring(&refresh_token);

                let exp_time = User::validate_token(&access_token).unwrap();
                let user_name = response_tokens["name"].as_str().unwrap_or("Unknown").to_string();

                Ok(User {
                    access_token,
                    refresh_token,
                    exp: exp_time,
                    companys: Vec::new(),
                    services: HashMap::new(),
                    is_save: true,
                    name: user_name,
                })
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "User not found",
            ))),
        }
    }

    pub async fn new(
        email: String,
        password: String,
        is_save: bool,
    ) -> Result<User, Box<dyn Error + Send + Sync>> {
        let url = "https://lk.2gc.ru/api/login/";
        let login_data = serde_json::json!({
            "email": email,
            "password": password
        });

        let client = Client::new();
        let client_info = get_client_info();
        let response = client
            .post(url)
            .header("User-Agent", USER_AGENT)
            .header("X-Client-Type", "desktop")
            .header("X-Client-Version", &client_info.version)
            .header("X-Client-OS", &client_info.os)
            .header("X-Client-Arch", &client_info.arch)
            .json(&login_data)
            .send()
            .await?;
        let response_status = response.status();
        let response_tokens: Value = response.json().await?;
        println!("[AUTH] Server response: {:?}", response_tokens);
        println!("[AUTH] Response status: {}", response_status);

        let access_token = response_tokens["access"]
            .as_str()
            .ok_or("Missing access token")?
            .to_string();

        let refresh_token = response_tokens["refresh"]
            .as_str()
            .ok_or("Missing refresh token")?
            .to_string();

        let user_name = response_tokens["name"].as_str().unwrap_or("Unknown").to_string();

        let exp_time = User::validate_token(&access_token).unwrap();

        if is_save {
            set_refresh_token_to_keyring(&refresh_token);
        } else {
            delete_refresh_token_from_keyring();
        }

        Ok(User {
            refresh_token,
            access_token,
            exp: exp_time,
            companys: Vec::new(),
            services: HashMap::new(),
            is_save,
            name: user_name,
        })
    }

    async fn update_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://lk.2gc.ru/api/token/refresh/";
        let login_refresh = serde_json::json!({
            "refresh": self.refresh_token.clone(),
        });
        let client = Client::new();
        let client_info = get_client_info();
        let response = client
            .post(url)
            .header("User-Agent", USER_AGENT)
            .header("X-Client-Type", "desktop")
            .header("X-Client-Version", &client_info.version)
            .header("X-Client-OS", &client_info.os)
            .header("X-Client-Arch", &client_info.arch)
            .json(&login_refresh)
            .send()
            .await?;
        let response_tokens: serde_json::Value = response.json().await?;
        println!("response {:?}", response_tokens);
        let access_token = response_tokens["access"].as_str().unwrap().to_string();
        let refresh_token = response_tokens["refresh"].as_str().unwrap().to_string();

        if self.is_save {
            set_refresh_token_to_keyring(&refresh_token);
        }

        let exp_time = User::validate_token(&access_token).unwrap();
        self.access_token = access_token;
        self.refresh_token = refresh_token;
        self.exp = exp_time;

        Ok(())
    }

    fn validate_token(access_token: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let key = DecodingKey::from_secret(&[]);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.insecure_disable_signature_validation();

        let token_data = decode::<Claims>(access_token, &key, &validation)?;
        Ok(token_data.claims.exp as u64)
    }

    pub async fn get_all_servers(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if self.exp < now || self.exp - now <= 60 {
            self.update_token().await?;
        }

        let url = "https://lk.2gc.ru/api/user/services/";
        let client = Client::new();
        let response = client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;
        let response_body: Vec<ServerResponse> = response.json().await?;

        for company in &response_body {
            for server in &company.servers {
                for service in &server.services {
                    self.services.insert(service.id.clone(), service.clone());
                }
            }
        }

        self.companys = response_body.clone();

        Ok(())
    }
}

pub async fn get_user() -> Option<User> {
    let user = USER.lock().await;
    user.clone()
}

pub async fn create_user(new_user: User) {
    let mut user = USER.lock().await;
    *user = Some(new_user);
}

pub async fn delete_user() {
    let mut user = USER.lock().await;
    *user = None;
}

pub async fn get_server_by_tunnel_id(tunnel_id: &str) -> Option<Service> {
    if let Some(mut usr) = get_user().await {
        let _ = usr.get_all_servers().await;
        let service = usr.services.get(tunnel_id);
        match service {
            Some(s) => Some(s.clone()),
            _ => None,
        }
    } else {
        None
    }
}