use common::bilibili::parse_bilibili_url;
use rbdc::datetime::FastDateTime;
use reqwest::header::HeaderMap;
use serde_json::Value;
use tauri::regex;
use crate::domain::mapper::live_info_mapper::select_live_info_by_condition;
use crate::domain::table::live_info::{LiveInfo, LiveRoomDetail};
use crate::RB;
use crate::rest_tool::{get_text, get_text_with_header};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Quality {
    pub desc: String,
    pub hdr_desc: String,
    pub quality: i32,
    url: String,
}
/// 获取虎牙直播间地址
#[tauri::command]
pub async fn get_huya_url(room_id: String) -> String {
    let url = format!("https://m.huya.com/{}", room_id);
    // 组装header
    let mut headers = HeaderMap::new();
    // headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1 Edg/91.0.4472.69".parse().unwrap());
    // get请求
    let result = get_text_with_header(url.as_str(), headers).await;
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
    unsafe {
        let vec
            = select_live_info_by_condition(&mut RB.get().unwrap(), room_id.as_str(), "虎牙直播").await.unwrap();
        // 如果 vec 为空
        if vec.is_empty(){
            // 插入直播
            let live_info = LiveInfo{
                id: None,
                name: Some(room_id.clone()),
                status: Some("1".into()),
                create_time: Some(FastDateTime::now()),
                room_id: Some(room_id.clone()),
                site_name: Some("虎牙直播".into()),
                site_url: Some("https://www.huya.com/".into()),
            };
            let data = LiveInfo::insert(
                &mut RB.get().unwrap(),
                &live_info
            ).await;
            println!("data: {:?}", data);
        }
    }
    println!("live_line_url: {}", live_line_url);
    common::huya::parse_huya_url(live_line_url)
}

/// 获取 bilibili 直播间地址
#[tauri::command()]
pub async fn get_bilibili_urls_with_quality(room_id: String) -> Vec<Quality> {

    let url = format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={}&protocol=0,1&format=0,1,2&codec=0,1&qn={}&platform=h5&ptype=8",
                      room_id, 10000);
    unsafe {
        let vec
            = select_live_info_by_condition(&mut RB.get().unwrap(), room_id.as_str(), "哔哩哔哩").await.unwrap();
        // 如果 vec 为空
        if vec.is_empty(){
            // 插入直播
            let live_info = LiveInfo{
                id: None,
                name: Some(room_id.clone()),
                status: Some("1".into()),
                create_time: Some(FastDateTime::now()),
                room_id: Some(room_id.clone()),
                site_name: Some("哔哩哔哩".into()),
                site_url: Some("https://live.bilibili.com/".into()),
            };
            let data = LiveInfo::insert(
                &mut RB.get().unwrap(),
                &live_info
            ).await;
            println!("data: {:?}", data);
        }
    }


    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (iPod; CPU iPhone OS 14_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/87.0.4280.163 Mobile/15E148 Safari/604.1".parse().unwrap());
    let result = get_text_with_header(url.as_str(), headers).await;
    // 获取清晰度
    let json: Value = serde_json::from_str(result.as_str()).unwrap();
    let quality_list = json["data"]["playurl_info"]["playurl"]["g_qn_desc"].as_array().unwrap();

    let mut bilibili_urls: Vec<Quality> = vec![];
    for quality in quality_list {
        println!("quality: {}", quality);
        let url = format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={}&protocol=0,1&format=0,1,2&codec=0,1&qn={}&platform=h5&ptype=8",
                          room_id, quality["qn"].as_i64().unwrap());
        let mut quality_struct = Quality {
            desc: quality["desc"].as_str().unwrap().to_string(),
            hdr_desc: quality["hdr_desc"].as_str().unwrap().to_string(),
            quality: quality["qn"].as_i64().unwrap() as i32,
            url: "".to_string(),
        };

        // 组装header
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (iPod; CPU iPhone OS 14_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/87.0.4280.163 Mobile/15E148 Safari/604.1".parse().unwrap());
        let result = get_text_with_header(url.as_str(), headers).await;
        let bilibili_url_list = parse_bilibili_url(result);
        quality_struct.url = bilibili_url_list.get(0).unwrap().to_string();
        bilibili_urls.push(quality_struct);
    }
    bilibili_urls
}

/// 获取 bilibili 直播间地址
#[tauri::command()]
pub async fn get_bilibili_url(room_id: String) -> String {

    let url = format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={}&protocol=0,1&format=0,1,2&codec=0,1&qn={}&platform=h5&ptype=8",
                      room_id, 10000);
    unsafe {
        let vec
            = select_live_info_by_condition(&mut RB.get().unwrap(), room_id.as_str(), "哔哩哔哩").await.unwrap();
        // 如果 vec 为空
        if vec.is_empty(){
            // 插入直播
            let live_info = LiveInfo{
                id: None,
                name: Some(room_id.clone()),
                status: Some("1".into()),
                create_time: Some(FastDateTime::now()),
                room_id: Some(room_id.clone()),
                site_name: Some("哔哩哔哩".into()),
                site_url: Some("https://live.bilibili.com/".into()),
            };
            let data = LiveInfo::insert(
                &mut RB.get().unwrap(),
                &live_info
            ).await;
            println!("data: {:?}", data);
        }
    }


    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (iPod; CPU iPhone OS 14_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/87.0.4280.163 Mobile/15E148 Safari/604.1".parse().unwrap());
    let result = get_text_with_header(url.as_str(), headers).await;
    // 获取清晰度
    let json: Value = serde_json::from_str(result.as_str()).unwrap();
    let quality_list = json["data"]["playurl_info"]["playurl"]["g_qn_desc"].as_array().unwrap();
    let url = format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={}&protocol=0,1&format=0,1,2&codec=0,1&qn={}&platform=h5&ptype=8",
                      room_id, quality_list[0]["qn"].as_i64().unwrap());
    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (iPod; CPU iPhone OS 14_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/87.0.4280.163 Mobile/15E148 Safari/604.1".parse().unwrap());
    let result = get_text_with_header(url.as_str(), headers).await;
    let bilibili_url_list = parse_bilibili_url(result);
    bilibili_url_list.get(0).unwrap().to_string()
}