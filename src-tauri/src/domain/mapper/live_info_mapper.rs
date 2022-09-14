use std::process::id;
use rbatis::executor::Executor;
use rbdc::datetime::FastDateTime;
use crate::domain::table::live_info::LiveInfo;
use crate::RB;

crud!(LiveInfo{},"live_info");

#[tauri::command]
pub async fn list_live_info() -> Vec<LiveInfo> {
    let mut list = vec![];
    unsafe {
        let live_info = LiveInfo::select_all(&mut RB.get().unwrap()).await;
        println!("live_info: {:?}", live_info);
        if let Ok(live_info) = live_info {
            list = live_info;
        }
    }
    list
}

#[tauri::command]
pub async fn del_live_info_by_id(id: i64) {
    unsafe {
        let result = LiveInfo::delete_by_column(&mut RB.get().unwrap(), "id", id).await.expect("no such data");
    }
}

#[tauri::command]
pub async fn add_live_info(mut live_info: LiveInfo) {
    live_info.create_time = Some(FastDateTime::now());
    unsafe {
        let vec
            = select_live_info_by_condition(&mut RB.get().unwrap(), live_info.room_id.as_ref().unwrap().as_str(), live_info.site_name.as_ref().unwrap().as_str()).await.unwrap();
        // 如果 vec 为空
        if vec.is_empty(){
            let result = LiveInfo::insert(&mut RB.get().unwrap(), &live_info).await.expect("add error");
        }
    }
}

#[html_sql("src/domain/mapper/html_mapper/live_info_mapper.html")]
pub async fn select_live_info_by_condition(rb: &mut dyn Executor, room_id: &str, site_name: &str) -> Vec<LiveInfo> {
    impled!()
}
