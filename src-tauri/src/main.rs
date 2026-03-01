#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use tauri::Manager;

fn get_data_path(app: &tauri::AppHandle) -> PathBuf {
    let data_dir = app.path_resolver().app_data_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    fs::create_dir_all(&data_dir).ok();
    data_dir.join("passover_data.json")
}

#[tauri::command]
fn load_data(app: tauri::AppHandle) -> String {
    let path = get_data_path(&app);
    fs::read_to_string(&path).unwrap_or_else(|_| String::from("null"))
}

#[tauri::command]
fn save_data(app: tauri::AppHandle, data: String) -> bool {
    let path = get_data_path(&app);
    fs::write(&path, data).is_ok()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_data, save_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
