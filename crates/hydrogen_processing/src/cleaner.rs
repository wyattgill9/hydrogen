use hydrogen_common::models::{CleanedData, RawHtmlData};
use regex::Regex;

pub async fn clean_data(raw_data: RawHtmlData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    let cleaned_html = RegexCleaner::new()
        .clean(&raw_data.raw_html)
        .finalize();

    Ok(CleanedData {
        source_url: raw_data.source_url,
        cleaned_html,
        timestamp: raw_data.timestamp,
    })
}

struct RegexCleaner {
    data: String,
}

impl RegexCleaner {
    fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    fn clean(mut self, input: &str) -> Self {
        println!("Cleaning data: {}", input);
        self
    }


    fn finalize(self) -> String {
        self.data
    }
}
