use hydrogen_common::models::RawHtmlData;

pub async fn clean_data(raw_data: RawHtmlData) -> Result<(), Box<dyn std::error::Error>> {
    println!("Cleaned data from URL: {}", raw_data.source_url);
    println!("Raw HTML: {}", raw_data.raw_html);
    Ok(())
}
