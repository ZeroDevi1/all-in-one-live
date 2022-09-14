#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod live;
mod rest_tool;
mod domain;


//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;
extern crate rbdc;

use crate::domain::{init_db, RB};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    let rbatis = init_db();
    unsafe { RB.set(rbatis.await).unwrap(); }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![live::get_huya_url,live::get_bilibili_url,crate::domain::mapper::live_info_mapper::list_live_info,crate::domain::mapper::live_info_mapper::del_live_info_by_id,crate::domain::mapper::live_info_mapper::add_live_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
