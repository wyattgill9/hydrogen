use hydrogen_common::models::{CleanedData, RawHtmlData};
use scraper::{Html, Selector};

pub async fn clean_data(raw_data: RawHtmlData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    let document = Html::parse_document(&raw_data.raw_html);

    let body_selector =
        Selector::parse("body").unwrap_or_else(|_| Selector::parse("html").unwrap());

    let cleaned_html = document
        .select(&body_selector)
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    Ok(CleanedData {
        source_url: raw_data.source_url,
        cleaned_html,
        timestamp: raw_data.timestamp,
    })
}
