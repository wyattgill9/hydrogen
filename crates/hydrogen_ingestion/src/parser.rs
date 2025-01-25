use hydrogen_common::models::HtmlData;

pub fn parse_html(raw_html: String, peer_addr: String) -> HtmlData {
    let html_data = HtmlData {
        source_url: format!("http://{}", peer_addr),
        raw_html,
        timestamp: chrono::Utc::now().timestamp() as u64,
    };
    html_data
}
