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

#[html_sql("src/domain/mapper/html_mapper/live_info_mapper.html")]
pub async fn select_live_info_by_condition(rb: &mut dyn Executor, room_id: &str, site_name: &str) -> Vec<LiveInfo> {
    impled!()
}