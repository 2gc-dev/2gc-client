use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_url: String,
    pub user_agent: String,
    pub max_retries: u32,
    pub retry_delay: u64,
    pub log_level: String,
    pub auto_reconnect: bool,
    pub check_updates: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:5021".to_string(),
            user_agent: "2GC-Desktop-Client".to_string(),
            max_retries: 3,
            retry_delay: 1,
            log_level: "info".to_string(),
            auto_reconnect: true,
            check_updates: true,
        }
    }
}

lazy_static! {
    static ref CONFIG: Arc<Mutex<AppConfig>> = Arc::new(Mutex::new(AppConfig::default()));
}

pub async fn load_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        info!("Конфигурационный файл не найден, создаем новый");
        let config = AppConfig::default();
        save_config(&config).await?;
        return Ok(());
    }

    let config_str = fs::read_to_string(&config_path)?;
    match serde_json::from_str::<AppConfig>(&config_str) {
        Ok(config) => {
            info!("Конфигурация успешно загружена");
            let mut current_config = CONFIG.lock().await;
            *current_config = config;
        }
        Err(e) => {
            error!("Ошибка при чтении конфигурации: {}", e);
            warn!("Используем конфигурацию по умолчанию");
        }
    }

    Ok(())
}

pub async fn save_config(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let config_str = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, config_str)?;
    info!("Конфигурация успешно сохранена");
    Ok(())
}

pub async fn get_config() -> AppConfig {
    CONFIG.lock().await.clone()
}

pub async fn update_config(updates: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = CONFIG.lock().await;
    *config = updates;
    save_config(&config).await
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = dirs::config_dir().ok_or("Не удалось получить путь к директории конфигурации")?;
    path.push("2gc-desktop");
    fs::create_dir_all(&path)?;
    path.push("config.json");
    Ok(path)
} 