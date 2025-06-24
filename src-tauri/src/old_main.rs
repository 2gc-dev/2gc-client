#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lazy_static::lazy_static;
use serde::{de::value, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

mod process;
mod rdp_settings;
mod remoute;
mod ssh_settings;
mod storage;
//mod utils;
use rdp_settings::{
    get_connect_params, get_or_create_rdp_params, save_connect_params, ConnectRDPParams,
};
use remoute::{delete_user, Server, ServerResponse, Service, User};
use ssh_settings::{get_or_create_ssh_params, ConnectSSHParams};
//use utils::{get_path_in_cach, create_data_folder, delete_temp_password_file, create_temp_password_file};
use process::{stop_all_process, ProcessType};
use storage as crypto_storage;
use tauri::GlobalWindowEvent;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::SystemTrayMenuItem;

fn main() {
    let is_open = Arc::new(Mutex::new(false)); // Создаем обертку для флага

    let quit = CustomMenuItem::new("open".to_string(), "Открыть");
    let hide = CustomMenuItem::new("quit".to_string(), "Закрыть");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .system_tray(tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user,
            auth_user,
            get_servers,
            set_connect_rdp,
            start_rdp,
            update_server_connect_params,
            try_remember_token,
            get_login_data,
            singout,
            clear_storage,
            start_ssh,
            set_connect_ssh
        ])
        .on_system_tray_event({
            let is_open = Arc::clone(&is_open); // Клонируем для использования в замыкании
            move |app, event| match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "open" => {
                            let mut open = is_open.lock().unwrap();
                            if !*open {
                                // Показываем окно, если оно скрыто
                                if let Some(window) = app.get_window("main") {
                                    window.show().unwrap();
                                    *open = true; // Устанавливаем флаг в true
                                }
                            }
                        }
                        "quit" => {
                            // Скрываем окно, если оно открыто
                            if let Some(window) = app.get_window("main") {
                                stop_all_process();
                                window.close().unwrap();
                                *is_open.lock().unwrap() = false; // Сбрасываем флаг
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}