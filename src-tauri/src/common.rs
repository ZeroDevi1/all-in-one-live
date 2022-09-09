use serde::{Serialize, Serializer, Deserialize};
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