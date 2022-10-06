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
    let data = format!("https:{}", live_line_url);
    // 根据 live_room_detail 的 data 获取清晰度
    // 把 data 中符合 .*?\.hls\.huya\.com 正则替换为 https://tx.hls.huya.com，选择线路
    let line_data = data.replace(r".*?\.hls\.huya\.com", "https://tx.hls.huya.com");
    common::huya::parse_huya_url(line_data)
}


/// 获取虎牙直播间地址
#[tauri::command]
pub async fn get_huya_urls_with_quality(room_id: String) -> Vec<Quality> {
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
    let mut huya_urls: Vec<Quality> = vec![];
    let data = format!("https:{}", live_line_url);
    // 根据 live_room_detail 的 data 获取清晰度
    // 把 data 中符合 .*?\.hls\.huya\.com 正则替换为 https://tx.hls.huya.com，选择线路
    // let line_data = data.replace(r".*?\.hls\.huya\.com", "https://tx.hls.huya.com");
    // 克隆 4份 line_data，分别对应 原画 原画HLS 高清 高清HLS
    let mut line_data_1 = data.clone();
    let mut line_data_2 = data.clone();
    let mut line_data_3 = data.clone();
    let mut line_data_4 = data.clone();
    // 原画替换 .*?\.hls\.huya\.com 为 https://tx.hls.huya.com
    line_data_1 = line_data_1.replace(r".*?\.hls\.huya\.com", "https://tx.hls.huya.com");
    // 原画HLS 替换 .*?\.hls\.huya\.com 为 https://bd.hls.huya.com
    line_data_2 = line_data_2.replace(r".*?\.hls\.huya\.com", "https://bd.hls.huya.com");
    // 高清 替换 .*?\.hls\.huya\.com 为 https://al.hls.huya.com
    line_data_3 = line_data_3.replace(r".*?\.hls\.huya\.com", "https://al.hls.huya.com");
    // 高清HLS 替换 .*?\.hls\.huya\.com 为 https://migu-bd.hls.huya.com
    line_data_4 = line_data_4.replace(r".*?\.hls\.huya\.com", "https://migu-bd.hls.huya.com");
    // 使用 parse_huya_url 分别解析
    let quality_1 = common::huya::parse_huya_url(line_data_1);
    let quality_2 = common::huya::parse_huya_url(line_data_2);
    let quality_3 = common::huya::parse_huya_url(line_data_3);
    let quality_4 = common::huya::parse_huya_url(line_data_4);
    // 把解析结果放入 huya_urls
    huya_urls.push(Quality { desc: "原画".to_string(), hdr_desc:"原画".to_string(), quality: 30000, url: quality_1 });
    huya_urls.push(Quality { desc: "原画HLS".to_string(), hdr_desc:"原画HLS".to_string(), quality: 30000, url: quality_2 });
    huya_urls.push(Quality { desc: "高清".to_string(), hdr_desc:"高清".to_string(), quality: 30000, url: quality_3 });
    huya_urls.push(Quality { desc: "高清HLS".to_string(), hdr_desc:"高清HLS".to_string(), quality: 30000, url: quality_4 });
    // common::huya::parse_huya_url(line_data);
    huya_urls
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

/// 获取斗鱼直播间地址
#[tauri::command]
pub async fn get_douyu_url(room_id: String) -> String {
    let result = format!("https://www.douyu.com/{}", room_id);
    let result = get_text(result.as_str()).await;
    // println!("result: {}", result);
    let url = format!("https://m.douyu.com/{}", room_id);
    // 组装header
    let mut headers = HeaderMap::new();
    // headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1 Edg/91.0.4472.69".parse().unwrap());
    // get请求
    let result = get_text_with_header(url.as_str(), headers).await;
    print!("result: {}", result);
    // "var $ROOM = {"rid":252140,"vipId":0,"roomName":"25262728休息 29号6点直播","nickname":"金咕咕金咕咕doinb","ownerId":16241112,"avatar":"https://apic.douyucdn.cn/up
    // load/avatar_v3/202004/df57d3efec57443994b960caec579364_middle.jpg","cate1Id":1,"cate2Id":1,"cate2Name":"英雄联盟","roomSrc":"https://rpic.douyucdn.cn/live-cover/roomCove
    // _coverUpdate_2022-09-15_a61614cd2656e3287e8d74269f85b815.jpg/dy1","roomSrcSixteen":"https://rpic.douyucdn.cn/live-cover/roomCove_coverUpdate_2022-09-15_a61614cd2656e3287
    // e8d74269f85b815.jpg/dy1","roomSrcBaidu":"https://rpic.douyucdn.cn/live-cover/roomCove_coverUpdate_2022-09-15_a61614cd2656e3287e8d74269f85b815.jpg/bd1","isVertical":0,"is
    // Live":0,"showTime":1664013452,"notice":"兄弟们有粉丝荧光棒的投喂一下 谢谢~\n欢迎加入我的车队”薄荷“，”谢谢老板一百个飞机”，”老板还有吗“，“咕家军“~","hn":"0","liveCity":"","isAudio":0,"isTicket":0}",
    // 使用正则查找 \$ROOM\s=\s{[\s\S]*}
    let re = regex::Regex::new(r"\$ROOM\s=\s{[\s\S]*}").unwrap();
    let room_info = re.find(result.as_str()).unwrap().as_str();
    println!("room_info: {}", room_info);

    unsafe {
        let vec
            = select_live_info_by_condition(&mut RB.get().unwrap(), room_id.as_str(), "斗鱼直播").await.unwrap();
        // 如果 vec 为空
        if vec.is_empty(){
            // 插入直播
            let live_info = LiveInfo{
                id: None,
                name: Some(room_id.clone()),
                status: Some("1".into()),
                create_time: Some(FastDateTime::now()),
                room_id: Some(room_id.clone()),
                site_name: Some("斗鱼直播".into()),
                site_url: Some("https://www.douyu.com/".into()),
            };
            let data = LiveInfo::insert(
                &mut RB.get().unwrap(),
                &live_info
            ).await;
            println!("data: {:?}", data);
        }
    }

    "1".to_string()
}


/// 获取斗鱼直播间地址
#[tauri::command]
pub async fn get_douyu_urls_with_quality(room_id: String) -> Vec<Quality> {
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
    let mut huya_urls: Vec<Quality> = vec![];
    let data = format!("https:{}", live_line_url);
    // 根据 live_room_detail 的 data 获取清晰度
    // 把 data 中符合 .*?\.hls\.huya\.com 正则替换为 https://tx.hls.huya.com，选择线路
    // let line_data = data.replace(r".*?\.hls\.huya\.com", "https://tx.hls.huya.com");
    // 克隆 4份 line_data，分别对应 原画 原画HLS 高清 高清HLS
    let mut line_data_1 = data.clone();
    let mut line_data_2 = data.clone();
    let mut line_data_3 = data.clone();
    let mut line_data_4 = data.clone();
    // 原画替换 .*?\.hls\.huya\.com 为 https://tx.hls.huya.com
    line_data_1 = line_data_1.replace(r".*?\.hls\.huya\.com", "https://tx.hls.huya.com");
    // 原画HLS 替换 .*?\.hls\.huya\.com 为 https://bd.hls.huya.com
    line_data_2 = line_data_2.replace(r".*?\.hls\.huya\.com", "https://bd.hls.huya.com");
    // 高清 替换 .*?\.hls\.huya\.com 为 https://al.hls.huya.com
    line_data_3 = line_data_3.replace(r".*?\.hls\.huya\.com", "https://al.hls.huya.com");
    // 高清HLS 替换 .*?\.hls\.huya\.com 为 https://migu-bd.hls.huya.com
    line_data_4 = line_data_4.replace(r".*?\.hls\.huya\.com", "https://migu-bd.hls.huya.com");
    // 使用 parse_huya_url 分别解析
    let quality_1 = common::huya::parse_huya_url(line_data_1);
    let quality_2 = common::huya::parse_huya_url(line_data_2);
    let quality_3 = common::huya::parse_huya_url(line_data_3);
    let quality_4 = common::huya::parse_huya_url(line_data_4);
    // 把解析结果放入 huya_urls
    huya_urls.push(Quality { desc: "原画".to_string(), hdr_desc:"原画".to_string(), quality: 30000, url: quality_1 });
    huya_urls.push(Quality { desc: "原画HLS".to_string(), hdr_desc:"原画HLS".to_string(), quality: 30000, url: quality_2 });
    huya_urls.push(Quality { desc: "高清".to_string(), hdr_desc:"高清".to_string(), quality: 30000, url: quality_3 });
    huya_urls.push(Quality { desc: "高清HLS".to_string(), hdr_desc:"高清HLS".to_string(), quality: 30000, url: quality_4 });
    // common::huya::parse_huya_url(line_data);
    huya_urls
}
