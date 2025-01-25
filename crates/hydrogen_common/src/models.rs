use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlData {
    pub source_url: String,
    pub raw_html: String,
    pub timestamp: u64,
}

impl Default for HtmlData {
    fn default() -> Self {
        HtmlData {
            source_url: "".to_string(),
            raw_html: "".to_string(),
            timestamp: 0,
        }
    }
}
