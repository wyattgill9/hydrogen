use serde::{Deserialize, Serialize};
// use std::clone;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawHtmlData {
    pub source_url: String,
    pub raw_html: String,
    pub timestamp: u64,
}

impl Default for RawHtmlData {
    fn default() -> RawHtmlData {
        RawHtmlData {
            source_url: "".to_string(),
            raw_html: "".to_string(),
            timestamp: 0,
        }
    }
}

impl From<CleanedData> for RawHtmlData {
    fn from(cleaned_data: CleanedData) -> Self {
        // convert CleanedData to RawHtmlData
        RawHtmlData {
            source_url: cleaned_data.source_url,
            raw_html: cleaned_data.cleaned_html,
            timestamp: cleaned_data.timestamp,
        }
    }
}

impl From<RawHtmlData> for CleanedData {
    fn from(raw_data: RawHtmlData) -> Self {
        // Convert raw to cleaned
        CleanedData {
            source_url: raw_data.source_url,
            cleaned_html: raw_data.raw_html,
            timestamp: raw_data.timestamp,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CleanedData {
    pub source_url: String,
    pub cleaned_html: String,
    pub timestamp: u64,
}

impl Default for CleanedData {
    fn default() -> CleanedData {
        CleanedData {
            source_url: "".to_string(),
            cleaned_html: "".to_string(),
            timestamp: 0,
        }
    }
}
