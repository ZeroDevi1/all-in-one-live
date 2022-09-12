#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
mod common_type;
mod model;
mod live;
mod rest_tool;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![live::get_huya_url,live::get_bilibili_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
