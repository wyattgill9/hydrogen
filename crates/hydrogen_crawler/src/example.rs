use hydrogen_common::models::RawHtmlData;

use reqwest::{Client, header};
use std::error::Error;
use chrono::Utc;

pub async fn crawler(url: &str) -> Result<RawHtmlData, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .pool_idle_timeout(Some(std::time::Duration::from_secs(30)))
        .pool_max_idle_per_host(5)
        .tcp_keepalive(std::time::Duration::from_secs(60))
        .build()?;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT_ENCODING,
        header::HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(
        header::CONNECTION,
        header::HeaderValue::from_static("keep-alive"),
    );
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        ),
    );

    let response = match client.get(url).headers(headers.clone()).send().await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Initial request failed: {}", e);
            println!("Retrying with HTTP/1.1 only...");
            let http1_client = Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .http1_only()
                .build()?;
            http1_client.get(url).headers(headers).send().await?
        }
    };

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;
    let html: String = String::from_utf8_lossy(&bytes).to_string(); 

    let now = Utc::now();
    let timestamp = now.timestamp() as u64;
    let raw_data = RawHtmlData {
        source_url: url.to_string(),
        raw_html: html,
        timestamp,
    };


    Ok(raw_data)
}
