// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use ipc::{begin_interrogation, case_intro};
use state::AppState;
use std::path::PathBuf;
pub mod case;
pub mod case_file;
pub mod difficulty;
pub mod error;
pub mod ids;
pub mod ipc;
pub mod state;
pub mod storage;
pub mod transcript;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new(PathBuf::from("cases")))
        .invoke_handler(tauri::generate_handler![case_intro, begin_interrogation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
