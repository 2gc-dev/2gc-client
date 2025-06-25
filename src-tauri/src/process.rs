// Импорты для всех платформ
use lazy_static::lazy_static;
use std::sync::Arc;
use tauri::{AppHandle};
use tauri_plugin_shell::{process::CommandChild, ShellExt};
use tokio::sync::Mutex;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
use std::io;
#[cfg(target_os = "windows")]
use std::env;
#[cfg(target_os = "windows")]
use std::process::{Command, Stdio};
#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use keyring::Entry;

#[cfg(target_os = "macos")]
use serde::{Deserialize, Serialize};
#[cfg(target_os = "macos")]
use confy;

// ------------------- STORAGE -------------------

#[cfg(target_os = "windows")]
pub async fn read_value_from_ini(section: &str, key: &str) -> Option<String> {
    let service = format!("2GC-{}", section);
    let entry = keyring::Entry::new(&service, key).ok()?;
    entry.get_password().ok()
}

#[cfg(target_os = "windows")]
pub async fn write_key_value_to_ini(section: &str, key: &str, value: &str) -> io::Result<()> {
    let service = format!("2GC-{}", section);
    let entry = keyring::Entry::new(&service, key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Keyring error: {}", e)))?;
    entry.set_password(value)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Keyring error: {}", e)))
}

#[cfg(target_os = "windows")]
pub async fn delete_key_from_ini(section: &str, key: &str) -> io::Result<()> {
    let service = format!("2GC-{}", section);
    let entry = keyring::Entry::new(&service, key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Keyring error: {}", e)))?;
    entry.delete_password().ok();
    Ok(())
}


#[cfg(target_os = "macos")]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StorageConfig {
    pub tockens: HashMap<String, String>,
    pub ports: HashMap<String, u16>,
}

#[cfg(target_os = "macos")]
impl StorageConfig {
    pub async fn load() -> Self {
        let app_name = "2GC_business".to_string();
        tokio::task::spawn_blocking(move || confy::load(&app_name, None).unwrap_or_default())
            .await
            .expect("Failed to load config")
    }
    pub async fn save(&self) {
        let app_name = "2GC_business".to_string();
        let config = self.clone();
        tokio::task::spawn_blocking(move || {
            confy::store(&app_name, None, config).expect("Failed to save config");
        })
        .await
        .expect("Failed to save config");
    }
}

#[cfg(target_os = "macos")]
pub async fn read_value_from_ini(section: &str, key: &str) -> Option<String> {
    let cfg = StorageConfig::load().await;
    if section == "auth" {
        cfg.tockens.get(key).cloned()
    } else if section == "port_storage" {
        cfg.ports.get(key).map(|v| v.to_string())
    } else {
        None
    }
}
#[cfg(target_os = "macos")]
pub async fn write_key_value_to_ini(section: &str, key: &str, value: &str) -> std::io::Result<()> {
    let mut cfg = StorageConfig::load().await;
    if section == "auth" {
        cfg.tockens.insert(key.to_string(), value.to_string());
    } else if section == "port_storage" {
        if let Ok(port) = value.parse::<u16>() {
            cfg.ports.insert(key.to_string(), port);
        }
    }
    cfg.save().await;
    Ok(())
}
#[cfg(target_os = "macos")]
pub async fn delete_key_from_ini(section: &str, key: &str) -> std::io::Result<()> {
    let mut cfg = StorageConfig::load().await;
    if section == "auth" {
        cfg.tockens.remove(key);
    } else if section == "port_storage" {
        cfg.ports.remove(key);
    }
    cfg.save().await;
    Ok(())
}

// ------------------- PROCESS -------------------

lazy_static! {
    static ref SERVER_PROCESS: Arc<Mutex<Vec<ServerProcess>>> = Arc::new(Mutex::new(Vec::new()));
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessType {
    Cloudflare,
    Rdp,
    Ssh,
}

#[derive(Debug, Clone)]
pub struct ServerProcess {
    pub cloudflare: ProcessHandler,
    pub rdp: ProcessHandler,
    pub ssh: ProcessHandler,
    pub tunnel_id: String,
}

impl ServerProcess {
    pub fn new(tunnel_id: &str) -> ServerProcess {
        ServerProcess {
            cloudflare: ProcessHandler::new(),
            rdp: ProcessHandler::new(),
            ssh: ProcessHandler::new(),
            tunnel_id: tunnel_id.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessHandler {
    child_processes: Arc<Mutex<Vec<CommandChild>>>,
}

impl ProcessHandler {
    pub fn new() -> Self {
        ProcessHandler {
            child_processes: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub async fn start_process<S: AsRef<str>>(
        &self,
        app_handle: &AppHandle,
        program: &str,
        args: Vec<S>,
    ) -> Result<String, String> {
        #[cfg(target_os = "windows")]
        {
            let shell = app_handle.shell();
            let (_, process) = shell
                .command(program)
                .args(args.iter().map(|arg| arg.as_ref()))
                .spawn()
                .map_err(|e| format!("Не удалось запустить процесс: {}", e))?;
            let mut processes = self.child_processes.lock().await;
            processes.push(process);
            Ok("Success".to_string())
        }
        #[cfg(target_os = "macos")]
        {
            let shell = app_handle.shell();
            let (_, process) = shell
                .sidecar(program)
                .unwrap()
                .args(args.iter().map(|arg| arg.as_ref()))
                .spawn()
                .map_err(|e| format!("Не удалось запустить процесс: {}", e))?;
            let mut processes = self.child_processes.lock().await;
            processes.push(process);
            Ok("Success".to_string())
        }
    }
    pub async fn stop_process(&self) {
        let mut processes = self.child_processes.lock().await;
        println!("Останавливаем {} процессов", processes.len());
        for mut process in processes.drain(..) {
            if let Err(e) = process.kill() {
                println!("Ошибка при остановке процесса: {:?}", e);
            } else {
                println!("Процесс успешно остановлен");
            }
        }
    }
    pub async fn is_process_running(&self) -> bool {
        let processes = self.child_processes.lock().await;
        !processes.is_empty()
    }
}

pub async fn get_or_create_process(tunnel_id: &str) -> ServerProcess {
    let mut settings = SERVER_PROCESS.lock().await;
    if let Some(process) = settings.iter_mut().find(|process| process.tunnel_id == tunnel_id) {
        process.clone()
    } else {
        let server_process = ServerProcess::new(tunnel_id);
        settings.push(server_process.clone());
        server_process
    }
}

pub async fn start_process_by_type(
    app_handle: &AppHandle,
    tunnel_id: &str,
    process_type: &ProcessType,
    args: Vec<String>,
) {
    let mut settings = SERVER_PROCESS.lock().await;
    if let Some(server_process) = settings.iter_mut().find(|process| process.tunnel_id == tunnel_id) {
        match process_type {
            ProcessType::Cloudflare => {
                server_process.rdp.stop_process().await;
                server_process.ssh.stop_process().await;
                #[cfg(target_os = "windows")]
                {
                    server_process
                        .cloudflare
                        .start_process(app_handle, get_program_path("cloudflared.exe").to_str().unwrap(), args)
                        .await
                        .unwrap();
                }
                #[cfg(target_os = "macos")]
                {
                    print!("Запускаем cloudflared с args: {:?}", args);
                    server_process
                        .cloudflare
                        .start_process(app_handle, "cloudflared", args)
                        .await
                        .unwrap();
                }
            }
            ProcessType::Rdp => {
                server_process.cloudflare.stop_process().await;
                server_process.ssh.stop_process().await;
                #[cfg(target_os = "windows")]
                {
                    server_process
                        .rdp
                        .start_process(app_handle, get_program_path("putty.exe").to_str().unwrap(), args)
                        .await
                        .unwrap();
                }
                #[cfg(target_os = "macos")]
                {
                    server_process
                        .rdp
                        .start_process(app_handle, "rdp", args)
                        .await
                        .unwrap();
                }
            }
            ProcessType::Ssh => {
                server_process.cloudflare.stop_process().await;
                server_process.rdp.stop_process().await;
                #[cfg(target_os = "windows")]
                {
                    server_process
                        .ssh
                        .start_process(app_handle, get_program_path("putty.exe").to_str().unwrap(), args)
                        .await
                        .unwrap();
                }
                #[cfg(target_os = "macos")]
                {
                    server_process
                        .ssh
                        .start_process(app_handle, "putty", args)
                        .await
                        .unwrap();
                }
            }
        }
    }
}

pub async fn stop_process_by_type(tunnel_id: &str, process_type: &ProcessType) {
    let mut settings = SERVER_PROCESS.lock().await;
    if let Some(server_process) = settings.iter_mut().find(|process| process.tunnel_id == tunnel_id) {
        match process_type {
            ProcessType::Cloudflare => {
                server_process.cloudflare.stop_process().await;
            }
            ProcessType::Rdp => {
                server_process.rdp.stop_process().await;
            }
            ProcessType::Ssh => {
                server_process.ssh.stop_process().await;
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn get_program_path(program: &str) -> PathBuf {
    let exe_path = env::current_exe().expect("Не удалось получить путь к исполняемому файлу");
    let exe_dir = exe_path.parent().expect("Не удалось получить директорию исполняемого файла");
    exe_dir.join("programs").join(program)
}

pub async fn get_free_port(key: &str) -> u16 {
    #[cfg(target_os = "windows")]
    {
        let section = "port_storage";
        if let Some(port_str) = read_value_from_ini(section, key).await {
            if let Ok(port) = port_str.parse::<u16>() {
                return port;
            }
        }
        for port in 1100..1200 {
            let port_str = port.to_string();
            let test_key = format!("check-{}", port);
            if read_value_from_ini(section, &test_key).await.is_none() {
                let _ = write_key_value_to_ini(section, key, &port_str).await;
                let _ = write_key_value_to_ini(section, &test_key, &port_str).await;
                return port;
            }
        }
        0
    }
    #[cfg(target_os = "macos")]
    {
        let key = key.to_string();
        let mut storage = StorageConfig::load().await;
        if let Some(port) = storage.ports.get(&key) {
            return *port;
        }
        let used_port: Vec<u16> = storage.ports.values().cloned().collect();
        for port in 1101..1200 {
            if !used_port.contains(&port) {
                storage.ports.insert(key.clone(), port);
                storage.save().await;
                return port;
            }
        }
        0
    }
}

pub async fn stop_all_cloudflared_processes(app_handle: &AppHandle) {
    println!("Останавливаем все cloudflared процессы");
    let settings = SERVER_PROCESS.lock().await;
    println!("Найдено {} серверных процессов", settings.len());
    for server_process in settings.iter() {
        println!("Останавливаем cloudflared для tunnel_id: {}", server_process.tunnel_id);
        server_process.cloudflare.stop_process().await;
    }
}

pub async fn stop_all_processes() {
    println!("Останавливаем все процессы");
    let settings = SERVER_PROCESS.lock().await;
    println!("Найдено {} серверных процессов", settings.len());
    for server_process in settings.iter() {
        println!("Останавливаем все процессы для tunnel_id: {}", server_process.tunnel_id);
        server_process.cloudflare.stop_process().await;
        server_process.rdp.stop_process().await;
        server_process.ssh.stop_process().await;
    }
}

#[cfg(target_os = "macos")]
pub async fn launch_rdp_client_with_rdp_file(
    tunnel_id: &str,
    host: &str,
    port: u16,
    username: &str,
    domain: Option<&str>,
) -> Result<(), String> {
    use std::fs;
    use std::process::Command;

    let mut rdp_content = format!(
        "full address:s:{}:{}\nusername:s:{}\nprompt for credentials:i:1\n",
        host, port, username
    );
    if let Some(domain_val) = domain {
        rdp_content += &format!("domain:s:{}\n", domain_val);
    }
    rdp_content += "\
screen mode id:i:2
use multimon:i:0
authentication level:i:2
compression:i:1
redirectclipboard:i:1
";
    let mut file_path = std::env::temp_dir();
    file_path.push(format!("{}_2gc.rdp", tunnel_id));
    fs::write(&file_path, rdp_content)
        .map_err(|e| format!("Не удалось сохранить RDP-файл: {}", e))?;
    let status = Command::new("open")
        .arg(file_path.to_str().unwrap_or(""))
        .status()
        .map_err(|e| format!("Не удалось открыть .rdp файл: {}", e))?;
    if !status.success() {
        return Err(format!("❌ Ошибка при запуске RDP-файла: {:?}", status));
    }
    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn clear_windows_credentials() -> io::Result<()> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    fn run_cmdkey_command(args: &[&str]) -> std::process::Output {
        Command::new("cmdkey")
            .args(args)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .expect("Failed to run cmdkey command")
    }
    let output = run_cmdkey_command(&["/list"]);
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        if let Some(target) = line.strip_prefix("    Target: ") {
            if target.contains("2GC") || target.contains("2gc") || target.contains("TERMSRV/127.0.0.1:") {
                let _ = run_cmdkey_command(&["/delete", target]);
            }
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
pub async fn clear_windows_credentials() -> std::io::Result<()> {
    // На macOS нет Windows Credential Manager, поэтому просто возвращаем Ok
    Ok(())
}

pub async fn clear_all_ports() {
    #[cfg(target_os = "windows")]
    {
        let section = "port_storage";
        // Удаляем все записи портов в диапазоне 1100-1200
        for port in 1100..1200 {
            let port_str = port.to_string();
            let test_key = format!("check-{}", port);
            let _ = delete_key_from_ini(section, &port_str).await;
            let _ = delete_key_from_ini(section, &test_key).await;
        }
    }
    #[cfg(target_os = "macos")]
    {
        // На macOS порты хранятся в StorageConfig, очистка происходит автоматически
        // при перезапуске приложения
    }
}

pub async fn clear_saved_credentials() {
    #[cfg(target_os = "windows")]
    {
        // Очищаем все секции, которые могут содержать сохраненные учетные данные
        // Секции создаются как последние 10 символов ID сервера
        for i in 0..1000 {
            let section = format!("{:?}", i);
            let _ = delete_key_from_ini(&section, "login").await;
        }
    }
    #[cfg(target_os = "macos")]
    {
        // На macOS учетные данные RDP/SSH не сохраняются в keyring
    }
}
