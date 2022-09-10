use reqwest::header::HeaderMap;
use serde_json::Value;
use tauri::regex;
use crate::common::LiveRoomDetail;



// /// 清晰度
// struct LivePlayQuality {
//     quality: String,
//     data: Any,
// }


/// 获取虎牙直播间地址
#[tauri::command]
pub async fn get_huya_url(room_id: String) -> String {
    let url = format!("https://m.huya.com/{}", room_id);
    // 创建 client
    let client = reqwest::Client::new();
    // 组装header
    let mut headers = HeaderMap::new();
    // headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1 Edg/91.0.4472.69".parse().unwrap());
    // get请求
    let result = client.get(&url)
        .headers(headers)
        .send().await.unwrap().text().await.unwrap();
    // 使用正则查找 "window\.HNF_GLOBAL_INIT.=.\{(.*?)\}.</script>",
    let re = regex::Regex::new(r#"window\.HNF_GLOBAL_INIT.=.\{(.*?)\}.</script>"#).unwrap();
    let cap = re.captures(&result).unwrap();
    // 把结果转换成json
    let json_str = format!("{{{}}}", cap[1].to_string());
    let json: Value = serde_json::from_str(&json_str).unwrap();
    //jsonObj["roomInfo"]["tLiveInfo"]["sRoomName"].ToString()
    // base64解码 json["roomProfile"]["liveLineUrl"],转换成 String
    let live_line_url = json["roomProfile"]["liveLineUrl"].as_str().unwrap();
    let live_line_url = base64::decode(live_line_url).unwrap();
    let live_line_url = String::from_utf8(live_line_url).unwrap();
    let live_room_detail = LiveRoomDetail {
        cover: json["roomInfo"]["tLiveInfo"]["sScreenshot"].to_string(),
        online: json["roomInfo"]["tLiveInfo"]["lTotalCount"].to_string(),
        room_id: json["roomInfo"]["tLiveInfo"]["lProfileRoom"].to_string(),
        title: json["roomInfo"]["tLiveInfo"]["sRoomName"].to_string(),
        user_name: json["roomInfo"]["tProfileInfo"]["sNick"].to_string(),
        user_avatar: json["roomInfo"]["tProfileInfo"]["sAvatar180"].to_string(),
        introduction: json["roomInfo"]["tLiveInfo"]["sIntroduction"].to_string(),
        notice: json["welcomeText"].to_string(),
        status: json["roomInfo"]["eLiveStatus"].to_string(),
        data: format!("https:{}", live_line_url),
        url,
    };
    huya::parse_huya_url(live_line_url)
}


