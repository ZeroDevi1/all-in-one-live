use std::any::Any;
use std::time::{SystemTime, UNIX_EPOCH};
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
    // let result = get_huya_real_url(room_id).await;
    // println!("result: {:?}", result);
    let url = format!("https://m.huya.com/{}", room_id);
    println!("url: {}", url);
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
    println!("result: {}", result);
    // 使用正则查找 "window\.HNF_GLOBAL_INIT.=.\{(.*?)\}.</script>",
    let re = regex::Regex::new(r#"window\.HNF_GLOBAL_INIT.=.\{(.*?)\}.</script>"#).unwrap();
    let cap = re.captures(&result).unwrap();
    // 把结果转换成json
    let json_str = format!("{{{}}}", cap[1].to_string());
    let json: Value = serde_json::from_str(&json_str).unwrap();
    println!("json: {}", json);
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
    // 根据 live_room_detail 的 data 获取清晰度
    // 把 data 中符合 .*?\.hls\.huya\.com 正则替换为 https://tx.hls.huya.com，选择线路
    let line_data = live_room_detail.data.replace(r".*?\.hls\.huya\.com", "https://tx.hls.huya.com");
    // 把 line_data 中的 hls.huya.com 替换为 flv.huya.com，_2000替换为"",ratio=2000&替换为"",.m3u8替换为.flv
    let line_data = line_data.replace("hls.huya.com", "flv.huya.com").replace("_2000", "").replace("ratio=2000&", "").replace(".m3u8", ".flv");
    // rust 通过正则替换 wsTime的值为631c39e6
    // let re = regex::Regex::new(r#"wsTime=(.*?)&"#).unwrap();
    // let line_data = re.replace_all(&line_data, "wsTime=631c39e6&").to_string();
    // 获取 fm 参数的值
    let fm = regex::Regex::new(r#"fm=(.*?)&"#).unwrap();
    let fm = fm.captures(&line_data).unwrap();
    // fm 转换成字符串
    let fm = fm[1].to_string();
    println!("fm: {}", fm);
    // 对 fm 进行 URI 解码
    let fm = percent_encoding::percent_decode_str(&fm).decode_utf8().unwrap().to_string();
    // 再对 fm 进行 base64 解码
    let fm = base64::decode(fm).unwrap();
    // fm 转换成字符串
    let fm = String::from_utf8(fm).unwrap();
    println!("fm: {}", fm);
    // fm 根据下划线拆分
    let fm = fm.split("_").collect::<Vec<&str>>();
    // 获取第1个值
    let p = fm[0].to_string();
    println!("p: {}", p);
    // 生成时间戳 *10000000
    let time:i64 = chrono::Local::now().timestamp() * 10000000;
    // let time:i64 = 16627078187016530;
    // 把 live_line_url 根据 ? 拆分，获取第0个的数据
    let live_line_url = live_line_url.split("?").collect::<Vec<&str>>()[0].to_string();
    // 然后根据 / 拆分，获取最后一个数据
    let live_line_url = live_line_url.split("/").collect::<Vec<&str>>().last().unwrap().to_string();
    // 把 live_line_url 的 .flv|.m3u8 替换为 ""
    let stream_name = live_line_url.replace(".flv", "").replace(".m3u8", "");
    // 获取 line_data 中 wsTime的值
    let ws_time = regex::Regex::new(r#"wsTime=(.*?)&"#).unwrap();
    let ws_time = ws_time.captures(&line_data).unwrap();
    let ws_time = ws_time[1].to_string();
    // 拼接新的 newWsSecret ${p}_0_${streamName}_${time}_${wsTime}
    let new_ws_secret = format!("{}_0_{}_{}_{}", p, stream_name, time, ws_time);
    // new_ws_secret md5 编码
    let new_ws_secret = md5::compute(new_ws_secret);
    // new_ws_secret 转换成字符串
    let new_ws_secret = format!("{:x}", new_ws_secret);
    println!("new_ws_secret: {}", new_ws_secret);
    // 通过正则替换 line_data 中的 wsSecret 的值
    let line_data = regex::Regex::new(r#"wsSecret=(.*?)&"#).unwrap().replace_all(&line_data, format!("wsSecret={}&", new_ws_secret).as_str()).to_string();
    // 添加 参数 seqid，值为 time
    let line_data = format!("{}&seqid={}", line_data, time);
    // 把 fm 参数和值通过正则删除
    let line_data = regex::Regex::new(r#"fm=(.*?)&"#).unwrap().replace_all(&line_data, "").to_string();
    // 把 ctype 参数和值通过正则删除
    let line_data = regex::Regex::new(r#"ctype=(.*?)&"#).unwrap().replace_all(&line_data, "").to_string();

    println!("line_data: {}", line_data);

    line_data
}


