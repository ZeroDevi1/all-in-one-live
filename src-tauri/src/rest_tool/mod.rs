use reqwest::get;
use reqwest::header::HeaderMap;

/// get请求，返回 text
/// # Arguments
/// * `url` - 请求地址
pub async fn get_text(url: &str) -> String {
    // 创建 client
    let client = reqwest::Client::new();
    // get请求
    client.get(url)
        .send().await.unwrap().text().await.unwrap()
}

/// get请求，返回 text
/// # Arguments
/// * `url` - 请求地址
/// * `headers` - 请求头
pub async fn get_text_with_header(url: &str,headers:HeaderMap) -> String {
    // 创建 client
    let client = reqwest::Client::new();
   // get请求
    client.get(url)
        .headers(headers)
        .send().await.unwrap().text().await.unwrap()
}