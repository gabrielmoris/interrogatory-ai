// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use ipc::case_intro;
pub mod case;
pub mod case_file;
pub mod difficulty;
pub mod error;
pub mod ids;
pub mod ipc;
pub mod storage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![case_intro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
