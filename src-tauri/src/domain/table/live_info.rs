use rbdc::datetime::FastDateTime;
use serde::{Serialize, Deserialize};

/// 直播间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveRoomDetail {
    pub cover: String,
    pub online: String,
    pub room_id: String,
    pub title: String,
    pub user_name: String,
    pub user_avatar: String,
    pub introduction: String,
    pub notice: String,
    pub status: String,
    pub data: String,
    pub url: String,
}

/// 直播间列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveInfo {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub site_name: Option<String>,
    pub site_url: Option<String>,
    pub room_id: Option<String>,
    pub status: Option<String>,
    pub create_time: Option<FastDateTime>,
}
