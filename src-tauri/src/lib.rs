mod auth;
mod connection;
mod wireguard;

use connection::ConnectionManager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ConnectionManager::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            auth::authenticate,
            wireguard::generate_keys,
            wireguard::connect_wireguard,
            connection::start_connection,
            connection::get_connection_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
