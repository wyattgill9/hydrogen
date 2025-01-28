use hydrogen_common::models::{CleanedData, RawHtmlData};
use regex::Regex;

pub async fn clean_data(raw_data: RawHtmlData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    let mut cleaned_html = raw_data.raw_html;

    cleaned_html = remove_tags_and_comments(&cleaned_html);

    cleaned_html = isolate_content(&cleaned_html);

    cleaned_html = clean_whitespace(&cleaned_html);

    println!("Cleaned HTML: {}", cleaned_html);

    Ok(CleanedData {
        source_url: raw_data.source_url,
        cleaned_html: cleaned_html.clone(),
    })
}

fn remove_tags_and_comments(html: &str) -> String {
    let regex1 = Regex::new(r"<!--.*?-->").unwrap();
    regex1.replace_all(html, "").to_string();

    let regex2 = Regex::new(r"<[^>]*>").unwrap();
    regex2.replace_all(html, "").to_string()
}

fn isolate_content(html: &str) -> String {
    let regex = Regex::new(r"(?s)<[^>]*>|[\s\t\r\n]+").unwrap();
    let isolate_content = regex.replace_all(html, " ").to_string();

    isolate_content
}

fn clean_whitespace(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut last_was_space = false;

    for c in html.bytes() {
        if c.is_ascii_whitespace() {
            if !last_was_space {
                result.push(' ');
                last_was_space = true;
            }
        } else {
            result.push(c as char);
            last_was_space = false;
        }
    }

    result.trim().to_string()
}
