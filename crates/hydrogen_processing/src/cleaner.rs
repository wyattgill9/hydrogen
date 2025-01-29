use hydrogen_common::models::{CleanedData, RawHtmlData};
use regex::Regex;

pub async fn clean_data(raw_data: RawHtmlData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    let cleaned_html = RegexCleaner::new()
        .remove_tags_and_comments(&raw_data.raw_html)
        .isolate_text()
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

    fn remove_tags_and_comments(mut self, input: &str) -> Self {
        let tag_comment_regex = Regex::new(r"<!--.*?-->|<[^>]*>").unwrap();
        self.data = tag_comment_regex.replace_all(input, " ").to_string();
        self
    }

    fn clean_whitespace(mut self) -> Self {
        let whitespace_regex = Regex::new(r"\s+").unwrap();
        self.data = whitespace_regex
            .replace_all(&self.data, " ")
            .trim()
            .to_string();
        self
    }

    fn isolate_text(mut self) -> Self {
        let text_regex = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
        self.data = text_regex.replace_all(&self.data, "").to_string();
        self
    }

    fn finalize(self) -> String {
        self.data
    }
}
