use hydrogen_common::models::CleanedData;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

pub async fn transform(data: CleanedData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.cleaned_html.as_bytes())?;
    let compressed_html = encoder.finish()?;

    let compressed_data = CleanedData {
        source_url: data.source_url,
        cleaned_html: String::from_utf8(compressed_html)?, 
        timestamp: data.timestamp,
    };

    Ok(compressed_data)
}