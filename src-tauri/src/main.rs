#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod domain;
mod live;
mod rest_tool;

//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;
extern crate rbdc;

use crate::domain::{init_db, RB};

#[tokio::main]
async fn main() {
    let rbatis = init_db();
    unsafe {
        RB.set(rbatis.await).unwrap();
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            live::get_huya_url,
            live::get_douyu_url,
            live::get_bilibili_url,
            live::get_bilibili_urls_with_quality,
            live::get_douyu_urls_with_quality,
            live::get_huya_urls_with_quality,
            crate::domain::mapper::live_info_mapper::list_live_info,
            crate::domain::mapper::live_info_mapper::del_live_info_by_id,
            crate::domain::mapper::live_info_mapper::add_live_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
