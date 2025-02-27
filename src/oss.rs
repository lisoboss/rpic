use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use chrono::{format::StrftimeItems, Datelike as _, Utc};
use hmac::{Hmac, Mac};
use md5::compute;
use reqwest::Client;
use sha1::Sha1;
use uuid::Uuid;

// 定义 HMAC-SHA1 类型
type HmacSha1 = Hmac<Sha1>;

// 用户登录名称 rpic@1318972003445929.onaliyun.com
const OSS_ACCESS_KEY_ID: &str = "LTAI5t75P2po9oxpoqGSMne8";
const OSS_ACCESS_KEY_SECRET: &str = "UYeclwCk13fWuOVeIs4gdbKSGtHkrB";
const OSS_BUCKET: &str = "rpic-d75c7cb3ffe8";
const OSS_ENDPOINT: &str = "oss-cn-chengdu.aliyuncs.com";

/// 生成 OSS Authorization 头
///
/// 参数说明：
/// - `access_key_id`：阿里云 OSS 的 AccessKeyId
/// - `access_key_secret`：阿里云 OSS 的 AccessKeySecret
/// - `verb`：请求方法（如 "PUT"、"GET" 等）
/// - `content_md5`：请求的 Content-MD5 值（如果没有可为空字符串）
/// - `content_type`：请求的 Content-Type
/// - `date`：请求的日期（符合 RFC 2616 格式）
/// - `canonicalized_oss_headers`：规范化的 OSS 头（如果无自定义头可为空字符串）
/// - `canonicalized_resource`：规范化的资源路径（格式为 "/bucket/object"）
fn generate_auth(
    verb: &str,
    content_md5: &str,
    content_type: &str,
    date: &str,
    canonicalized_oss_headers: &str,
    canonicalized_resource: &str,
) -> Result<String> {
    // 构造待签名字符串
    let string_to_sign = format!(
        "{verb}\n{content_md5}\n{content_type}\n{date}\n{canonicalized_oss_headers}{canonicalized_resource}"
    );

    // 使用 HMAC-SHA1 算法计算签名
    let mut mac = HmacSha1::new_from_slice(OSS_ACCESS_KEY_SECRET.as_bytes())?;
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize().into_bytes();

    let signature = general_purpose::STANDARD.encode(&result);

    Ok(format!("OSS {}:{}", OSS_ACCESS_KEY_ID, signature))
}

fn generate_file_path() -> String {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    let uuid = Uuid::new_v4();

    format!("{year}/{month:02}/{uuid}.webp")
}

fn generate_content_md5(input: &[u8]) -> String {
    let digest = compute(input);
    general_purpose::STANDARD.encode(digest.as_ref())
}

fn generate_http_date() -> String {
    let now = Utc::now();
    let format = StrftimeItems::new("%a, %d %b %Y %H:%M:%S GMT");
    now.format_with_items(format).to_string()
}

pub async fn put_webp(webp_data: &[u8]) -> Result<String> {
    let file_path = generate_file_path();

    let url = format!("https://{OSS_BUCKET}.{OSS_ENDPOINT}/{file_path}");

    let cache_control = "public";
    let content_md5 = generate_content_md5(webp_data);
    let content_type = "image/webp";
    let x_oss_object_acl = "public-read";
    let date = generate_http_date();
    let canonicalized_oss_headers = format!("x-oss-object-acl:{x_oss_object_acl}\n");
    let canonicalized_resource = format!("/{OSS_BUCKET}/{file_path}");

    let auth = generate_auth(
        "PUT",
        &content_md5,
        content_type,
        &date,
        &canonicalized_oss_headers,
        &canonicalized_resource,
    )?;

    let response = Client::new()
        .put(&url)
        .header("authorization", auth)
        .header("cache-control", cache_control)
        .header("content-md5", content_md5)
        .header("content-type", content_type)
        .header("date", date)
        .header("x-oss-object-acl", x_oss_object_acl)
        .body(webp_data.to_vec())
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if !status.is_success() {
        anyhow::bail!("oss put error status {} text: {}", status, text)
    }

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_auth() {
        // 示例参数（请根据实际情况替换）
        let verb = "PUT";
        let content_md5 = "eB5eJF1ptWaXm4bijSPyxw=="; // 如果有 MD5 值则填写，无则为空
        let content_type = "image/jpeg";
        let date = "Tue, 27 Feb 2025 10:00:00 GMT";
        let canonicalized_oss_headers = ""; // 若有自定义 OSS 头则填写
        let canonicalized_resource = "/your-bucket-name/your-object-key";

        let auth_header = generate_auth(
            verb,
            content_md5,
            content_type,
            date,
            canonicalized_oss_headers,
            canonicalized_resource,
        );

        println!("生成的 Authorization 头:\n{}", auth_header.unwrap());
    }

    #[test]
    fn test_content_md5() {
        let v = generate_content_md5("0123456789".as_bytes());
        assert_eq!(String::from("eB5eJF1ptWaXm4bijSPyxw=="), v);
    }

    #[tokio::test]
    async fn test_put_webp() {
        let content = b"123";
        let url = put_webp(content).await.unwrap();
        println!("{url}");
    }
}
