pub mod huya {
    pub fn parse_huya_url(url: String) -> String {
        let data = format!("https:{}", url);
        // 根据 live_room_detail 的 data 获取清晰度
        // 把 data 中符合 .*?\.hls\.common\.com 正则替换为 https://tx.hls.huya.com，选择线路
        let line_data = data.replace(r".*?\.hls\.common\.com", "https://tx.hls.huya.com");
        // 把 line_data 中的 hls.common.com 替换为 flv.common.com，_2000替换为"",ratio=2000&替换为"",.m3u8替换为.flv
        let line_data = line_data.replace("hls.common.com", "flv.common.com").replace("_2000", "").replace("ratio=2000&", "").replace(".m3u8", ".flv");
        // rust 通过正则替换 wsTime的值为631c39e6
        // let re = regex::Regex::new(r#"wsTime=(.*?)&"#).unwrap();
        // let line_data = re.replace_all(&line_data, "wsTime=631c39e6&").to_string();
        // 获取 fm 参数的值
        let fm = regex::Regex::new(r#"fm=(.*?)&"#).unwrap();
        let fm = fm.captures(&line_data).unwrap();
        // fm 转换成字符串
        let fm = fm[1].to_string();
        // 对 fm 进行 URI 解码
        let fm = percent_encoding::percent_decode_str(&fm).decode_utf8().unwrap().to_string();
        // 再对 fm 进行 base64 解码
        let fm = base64::decode(fm).unwrap();
        // fm 转换成字符串
        let fm = String::from_utf8(fm).unwrap();
        // fm 根据下划线拆分
        let fm = fm.split("_").collect::<Vec<&str>>();
        // 获取第1个值
        let p = fm[0].to_string();
        // 生成时间戳 *10000000
        let time: i64 = chrono::Local::now().timestamp() * 10000000;
        // let time:i64 = 16627078187016530;
        // 把 live_line_url 根据 ? 拆分，获取第0个的数据
        let live_line_url = url.split("?").collect::<Vec<&str>>()[0].to_string();
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
        // 通过正则替换 line_data 中的 wsSecret 的值
        let line_data = regex::Regex::new(r#"wsSecret=(.*?)&"#).unwrap().replace_all(&line_data, format!("wsSecret={}&", new_ws_secret).as_str()).to_string();
        // 添加 参数 seqid，值为 time
        let line_data = format!("{}&seqid={}", line_data, time);
        // 把 fm 参数和值通过正则删除
        let line_data = regex::Regex::new(r#"fm=(.*?)&"#).unwrap().replace_all(&line_data, "").to_string();
        // 把 ctype 参数和值通过正则删除
        let line_data = regex::Regex::new(r#"ctype=(.*?)&"#).unwrap().replace_all(&line_data, "").to_string();
        line_data
    }
}

pub mod bilibili {
    use serde_json::Value;
    pub fn parse_bilibili_url(result:String) -> Vec<String> {
        let mut url_list: Vec<String> = Vec::new();
        let json: Value = serde_json::from_str(result.as_str()).unwrap();
        json["data"]["playurl_info"]["playurl"]["stream"].as_array().unwrap().iter().for_each(|item| {
            let format_name = item["format"][0]["format_name"].as_str().unwrap();
            if format_name == "ts" {
                let base_url = item["format"][0]["codec"][0]["base_url"].as_str().unwrap();
                let url_info = item["format"][0]["codec"][0]["url_info"].as_array().unwrap();
                url_info.iter().for_each(|ele| {
                    let host = ele["host"].as_str().unwrap();
                    let extra = ele["extra"].as_str().unwrap();
                    let url = format!("{}{}{}", host, base_url, extra);
                    url_list.push(url);
                });
            }
        });
        url_list
    }
}