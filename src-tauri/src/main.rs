#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle};
use tauri::Manager;
use std::fs;
use tungstenite::{connect, Message};
use url::Url;
use std::{thread, time::Duration};
use tauri::Emitter;
use tauri::EventTarget;
use tungstenite::client::client;
use native_tls::TlsConnector;
use std::net::TcpStream;
use tungstenite::client::IntoClientRequest;

mod process;
mod rdp_settings;
mod remoute;
mod ssh_settings;
mod storage;

use rdp_settings::{get_or_create_rdp_params, ConnectRDPParams};
use remoute::{delete_user, ServerResponse, User};
use ssh_settings::{get_or_create_ssh_params, ConnectSSHParams};
use storage as crypto_storage;
use tauri_plugin_single_instance::init as single_instance;
use tauri_plugin_os;
use std::path::PathBuf;




#[cfg(target_os = "windows")]
mod platform {
    use std::process::Command;
    use encoding::{Encoding, DecoderTrap};
    use encoding::all::IBM866;
    pub const CREATE_NO_WINDOW: u32 = 0x08000000;
    use std::os::windows::process::CommandExt;

    pub fn decode_cp866(bytes: &[u8]) -> String {
        IBM866.decode(bytes, DecoderTrap::Replace).unwrap_or_else(|_| "<Invalid CP866 Output>".to_string())
    }
    pub fn run_cmdkey_command(args: &[&str]) -> std::process::Output {
        Command::new("cmd")
            .args(&["/C", "cmdkey"])
            .args(args)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .expect("Failed to run cmdkey via cmd /C")
    }
}

#[cfg(target_os = "macos")]
mod platform {
    // Если понадобятся специфичные для Mac методы — добавь сюда
    // Например, запуск Microsoft Remote Desktop через open
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
async fn get_user() -> Option<User> {
    remoute::get_user().await
}

#[tauri::command]
async fn auth_user(email: String, password: String, remember: bool) -> bool {
    let new_user = User::new(email, password, remember).await;
    match new_user {
        Ok(user) => {
            let access_token = user.access_token.clone();
            remoute::create_user(user).await;
            tokio::spawn(async move {
                if let Err(e) = remoute::collect_and_send_info_with_token(&access_token).await {
                    // Ошибка отправки системной информации
                }
            });
            true
        }
        Err(err) => {
            // Ошибка входа
            false
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceStatus {
    pub rdp_param: ConnectRDPParams,
    pub ssh_param: ConnectSSHParams,
    pub is_connected: bool,
    pub is_rdp: bool,
    pub is_ssh: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MainInfo {
    pub companys: Vec<ServerResponse>,
    pub service_status: HashMap<String, ServiceStatus>,
}

#[tauri::command]
async fn get_servers() -> MainInfo {
    let user = remoute::get_user().await;
    let mut result = MainInfo {
        companys: Vec::new(),
        service_status: HashMap::new(),
    };

    match user {
        Some(mut user_data) => {
            _ = user_data.get_all_servers().await;
            result.companys = user_data.companys.clone();
            for service in user_data.services.values() {
                let rdp_param = rdp_settings::get_or_create_rdp_params(service.id.as_str()).await;
                let ssh_param = ssh_settings::get_or_create_ssh_params(service.id.as_str()).await;
                let main_process = process::get_or_create_process(service.id.as_str()).await;
                let is_connected = main_process.cloudflare.is_process_running().await;
                let is_rdp = main_process.rdp.is_process_running().await;
                let is_ssh = main_process.ssh.is_process_running().await;

                let service_status = ServiceStatus {
                    rdp_param,
                    ssh_param,
                    is_connected,
                    is_rdp,
                    is_ssh,
                };
                result
                    .service_status
                    .insert(service.id.clone(), service_status);
            }
        }
        _ => {}
    }

    result
}

#[tauri::command]
async fn try_remember_token() -> bool {
    let new_user = User::try_remember().await;
    match new_user {
        Ok(user) => {
            remoute::create_user(user).await;
            true
        }
        _ => false,
    }
}

#[tauri::command]
async fn singout() {
    // Очищаем Windows Credential Manager
    let _ = process::clear_windows_credentials().await;
    // Даем системе время завершить удаление ключей
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    // Останавливаем все процессы
    process::stop_all_processes().await;
    // Очищаем выделенные порты
    process::clear_all_ports().await;
    // Очищаем сохраненные учетные данные RDP/SSH
    process::clear_saved_credentials().await;
    // Очищаем сохраненные учетные данные из keyring
    let _ = crypto_storage::clear_file().await;
    // Удаляем пользователя
    delete_user().await;
}

#[tauri::command]
async fn clear_storage() {
    let result = crypto_storage::clear_file().await;
    match result {
        Ok(_) => {},
        Err(e) => {},
    }
}

#[tauri::command]
async fn set_connect_rdp(app_handle: AppHandle, tunnelid: String, isstarted: bool) {
    let _server_process = process::get_or_create_process(&tunnelid).await;
    let process_type = process::ProcessType::Cloudflare;
    let server_info = remoute::get_server_by_tunnel_id(&tunnelid).await;
    let rdp_params = get_or_create_rdp_params(&tunnelid).await;
    match server_info {
        Some(srv) => {
            if isstarted {
                let mut prms = Vec::new();
                prms.push("access".to_string());
                prms.push("rdp".to_string());
                prms.push("--hostname".to_string());
                prms.push(srv.access_url.to_string());
                prms.push("--url".to_string());
                prms.push(format!("127.0.0.1:{}", rdp_params.localport));
                process::start_process_by_type(&app_handle, &tunnelid, &process_type, prms).await;
            } else {
                process::stop_process_by_type(&tunnelid, &process_type).await;
            }
        }
        _ => {}
    }
}

#[tauri::command]
async fn set_connect_ssh(app_handle: AppHandle, tunnelid: String, isstarted: bool) {
    let _server_process = process::get_or_create_process(&tunnelid).await;
    let process_type = process::ProcessType::Cloudflare;
    let server_info = remoute::get_server_by_tunnel_id(&tunnelid).await;
    let ssh_params = get_or_create_ssh_params(&tunnelid).await;

    match server_info {
        Some(srv) => {
            if isstarted {
                let mut prms = Vec::new();
                prms.push("access".to_string());
                prms.push("ssh".to_string());
                prms.push("--hostname".to_string());
                prms.push(srv.access_url.to_string());
                prms.push("--url".to_string());
                prms.push(format!("127.0.0.1:{}", ssh_params.localport));
                process::start_process_by_type(&app_handle, &tunnelid, &process_type, prms).await;
            } else {
                process::stop_process_by_type(&tunnelid, &process_type).await;
            }
        }
        _ => {}
    }
}

// ========== Платформозависимые функции и команды ===========

#[tauri::command]
async fn start_rdp(
    app_handle: AppHandle,
    tunnelid: String,
    username: String,
    password: String,
    domain: String,
    remember: bool,
) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use crate::platform::*;
        use std::fs;
        use std::process::Command;
        use tokio::time::{sleep, Duration};
        use std::path::Path;

        let server_process = process::get_or_create_process(&tunnelid).await;
        let server_info = remoute::get_server_by_tunnel_id(&tunnelid).await;
        let rdp_params = get_or_create_rdp_params(&tunnelid).await;

        if let Some(srv) = server_info {
            if !server_process.cloudflare.is_process_running().await {
                let prms = vec![
                    "access".to_string(),
                    "rdp".to_string(),
                    "--hostname".to_string(),
                    srv.access_url,
                    "--url".to_string(),
                    format!("rdp://127.0.0.1:{}", rdp_params.localport),
                ];
                process::start_process_by_type(
                    &app_handle,
                    &tunnelid,
                    &process::ProcessType::Cloudflare,
                    prms,
                ).await;
                sleep(Duration::from_secs(2)).await;
            }

            // Запоминаем логин/пароль если надо
            if remember {
                let key = "login";
                let value = format!("{}!-A-!{}!-A-!{}", username, password, domain);
                let last_10: String = srv.id.chars().rev().take(10).collect::<Vec<_>>().iter().rev().collect();
                let section = format!("{:?}", last_10);
                _ = crypto_storage::write_key_value_to_ini(&section, key, &value).await;
            }

            // Работа с cmdkey (чистим старое, создаём новое)
            let full_user = if domain.is_empty() {
                username.to_string()
            } else {
                format!("{}\\{}", domain, username)
            };
            let target = format!("TERMSRV/127.0.0.1:{}", rdp_params.localport);
            let _ = run_cmdkey_command(&["/delete", &target]);
            let _ = run_cmdkey_command(&["/add", &target, &format!("/user:{}", full_user), &format!("/pass:{}", password)]);

            // .rdp файл
            let rdp_file_path = format!("C:\\Users\\Public\\2gc_auto_{}.rdp", tunnelid);
            let rdp_content = format!(
r#"full address:s:127.0.0.1:{}
username:s:{}
domain:s:{}
redirectprinters:i:1
redirectdrives:i:1
audiomode:i:0
disable wallpaper:i:0
screen mode id:i:2
enablecredsspsupport:i:1
prompt for credentials:i:0
authentication level:i:0
"#,
                rdp_params.localport, full_user, domain
            );
            fs::write(&rdp_file_path, rdp_content).expect("Unable to write .rdp file");
            let mut mstsc_process = Command::new("mstsc")
                .arg(&rdp_file_path)
                .spawn()
                .expect("Failed to start mstsc");
            let _result = mstsc_process.wait().expect("Failed to wait for mstsc");
            // Удаляем RDP-файл после завершения сессии
            let path = Path::new(&rdp_file_path);
            if path.exists() { let _ = fs::remove_file(&path); }
            return Ok("RDP session started and file cleaned up".to_string());
        } else {
            return Err("Server information not found".to_string());
        }
    }
    #[cfg(target_os = "macos")]
    {
        // На macOS просто генерируем .rdp и открываем его через open (Microsoft Remote Desktop)
        use std::fs;
        use std::process::Command;
        use std::env::temp_dir;

        let rdp_params = get_or_create_rdp_params(&tunnelid).await;
        let rdp_file_path = temp_dir().join(format!("2gc_auto_{}.rdp", tunnelid));
        let full_user = if domain.is_empty() { username.clone() } else { format!("{}\\{}", domain, username) };
        let rdp_content = format!(
r#"full address:s:127.0.0.1:{}
username:s:{}
domain:s:{}
redirectprinters:i:1
redirectdrives:i:1
audiomode:i:0
disable wallpaper:i:0
screen mode id:i:2
enablecredsspsupport:i:1
prompt for credentials:i:0
authentication level:i:0
"#,
            rdp_params.localport, full_user, domain
        );
        fs::write(&rdp_file_path, rdp_content).map_err(|e| e.to_string())?;
        Command::new("open")
            .arg(&rdp_file_path)
            .spawn()
            .map_err(|e| format!("Failed to open RDP file: {}", e))?;
        Ok("RDP session started (macOS)".to_string())
    }
}

#[tauri::command]
fn run_updater() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::env;
        use std::process::Command;
        let exe_path = env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?;
        let exe_dir = exe_path.parent().ok_or("Failed to get parent dir")?;
        let updater_path = exe_dir.join("updater.exe");
        Command::new(&updater_path).arg("/checknow").spawn().map_err(|e| format!("Failed to launch updater: {}", e))?;
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("Updater supported only on Windows".to_string())
    }
}

#[tauri::command]
fn load_settings(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let app_dir = app_handle
        .path()
        .app_config_dir()  // <-- уже Result<PathBuf, Error>
        .map_err(|e| format!("Не удалось получить app_config_dir: {}", e))?;
    let settings_path = app_dir.join("settings.json");

    if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)
            .map_err(|e| format!("Ошибка чтения файла настроек: {}", e))?;
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;
        Ok(json)
    } else {
        Ok(serde_json::json!({}))
    }
}

#[tauri::command]
fn save_settings(app_handle: tauri::AppHandle, settings: serde_json::Value) -> Result<(), String> {
    let app_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Не удалось получить app_config_dir: {}", e))?;
    std::fs::create_dir_all(&app_dir).map_err(|e| format!("Ошибка создания папки: {}", e))?;
    let settings_path = app_dir.join("settings.json");
    std::fs::write(&settings_path, settings.to_string())
        .map_err(|e| format!("Ошибка записи файла настроек: {}", e))
}

#[tauri::command]
fn start_ws_listener(app_handle: tauri::AppHandle, token: String) {
    thread::spawn(move || {
        let mut retry_secs = 1;
        loop {
            let url = format!("wss://lk.2gc.ru/ws/notifications/?token={}&x-client-type=desktop", token);
            let domain = "lk.2gc.ru";
            let connector = TlsConnector::new().unwrap();
            match TcpStream::connect((domain, 443)) {
                Ok(stream) => {
                    match connector.connect(domain, stream) {
                        Ok(tls_stream) => {
                            // Создаём request с User-Agent
                            let mut request = url.into_client_request().unwrap();
                            request.headers_mut().append("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36".parse().unwrap());
                            
                            match client(request, tls_stream) {
                                Ok((mut socket, response)) => {
                                    retry_secs = 1;
                                    loop {
                                        match socket.read() {
                                            Ok(msg) => {
                                                if let Message::Text(txt) = msg {
                                                    match serde_json::from_str::<serde_json::Value>(&txt) {
                                                        Ok(val) => {
                                                            if let Err(e) = app_handle.emit("push-notification", val) {
                                                                // Emit error
                                                            }
                                                        }
                                                        Err(e) => {
                                                            // JSON parse error
                                                        }
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                // Read error
                                                break;
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    // WebSocket client error
                                }
                            }
                        }
                        Err(e) => {
                            // TLS connect error
                        }
                    }
                }
                Err(e) => {
                    // TCP connect error
                }
            }
            thread::sleep(Duration::from_secs(retry_secs));
            retry_secs = (retry_secs * 2).min(60);
        }
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(single_instance(|app, _argv, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user,
            auth_user,
            get_servers,
            set_connect_rdp,
            set_connect_ssh,
            start_rdp,
            try_remember_token,
            singout,
            clear_storage,
            run_updater,
            load_settings,
            save_settings,
            start_ws_listener,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                ctrlc::set_handler(move || {
                    tauri::async_runtime::block_on(async {
                        // Останавливаем только процессы при Ctrl+C
                        crate::process::stop_all_cloudflared_processes(&app_handle).await;
                        crate::process::stop_all_processes().await;
                    });
                    std::process::exit(0);
                }).expect("Failed to set Ctrl-C handler");
            });

            #[cfg(target_os = "macos")]
            {
                // Создаём простое бар-меню (верхнее меню macOS)
                let about = MenuItemBuilder::new("О программе")
                    .build(app)
                    .unwrap();
                let quit = MenuItemBuilder::new("Выйти")
                    .build(app)
                    .unwrap();

                let menu = MenuBuilder::new(app)
                    .item(&about)
                    .item(&quit)
                    .build()
                    .unwrap();

                app.set_menu(menu).unwrap();
            }

            // ...setup для трея, ctrlc и т.д...
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                println!("Окно закрывается, останавливаем процессы...");
                let app_handle = window.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    // Останавливаем только процессы при закрытии окна
                    println!("Вызываем stop_all_cloudflared_processes");
                    crate::process::stop_all_cloudflared_processes(&app_handle).await;
                    println!("Вызываем stop_all_processes");
                    crate::process::stop_all_processes().await;
                    println!("Все процессы остановлены, выходим");
                    std::process::exit(0);
                });
            }
        })
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
