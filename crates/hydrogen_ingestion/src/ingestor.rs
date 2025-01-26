use hydrogen_common::models::RawHtmlData;

pub fn ingest_data(raw_data: RawHtmlData) {
    println!("Ingested data from URL: {}", raw_data.source_url);
    println!("{}", raw_data.raw_html);
}
