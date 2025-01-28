use hydrogen_common::models::{RawHtmlData, CleanedData};

pub async fn clean_data(raw_data: RawHtmlData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    let mut cleaned_html = raw_data.raw_html;

    cleaned_html = remove_comments(&cleaned_html);

    cleaned_html = remove_tags_and_content(&cleaned_html, "script");

    cleaned_html = remove_tags_and_content(&cleaned_html, "style");

    cleaned_html = clean_whitespace(&cleaned_html);

    println!("Cleaned HTML: {}", cleaned_html);
    
    Ok(CleanedData {
        source_url: raw_data.source_url,
        cleaned_html: cleaned_html.clone(),
    })
}


fn remove_comments(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();
    
    while let Some(&c) = chars.peek() {
        if c == '<' && chars.clone().nth(1) == Some('!') && 
           chars.clone().nth(2) == Some('-') && chars.clone().nth(3) == Some('-') {
            while let Some(ch) = chars.next() {
                if ch == '-' && chars.clone().peek() == Some(&'-') && 
                   chars.clone().nth(1) == Some('>') {
                    chars.next();
                    chars.next();
                    break;
                }
            }
        } else {
            result.push(c);
            chars.next();
        }
    }
    
    result
}

fn remove_tags_and_content(html: &str, tag_name: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();
    
    while let Some(&c) = chars.peek() {
        if c == '<' {
            let mut tag = String::new();
            let mut peek = chars.clone();
            peek.next(); 
            
            while let Some(&ch) = peek.peek() {
                if ch.is_whitespace() || ch == '>' {
                    break;
                }
                tag.push(ch);
                peek.next();
            }
            
            if tag.eq_ignore_ascii_case(tag_name) {
                let mut depth = 1;
                while let Some(ch) = chars.next() {
                    if ch == '<' {
                        if chars.clone().peek() == Some(&'/') {
                            let mut close_tag = String::new();
                            chars.next(); 
                            while let Some(&ch) = chars.peek() {
                                if ch.is_whitespace() || ch == '>' {
                                    break;
                                }
                                close_tag.push(ch);
                                chars.next();
                            }
                            if close_tag.eq_ignore_ascii_case(tag_name) {
                                depth -= 1;
                                if depth == 0 {
                                    while let Some(&ch) = chars.peek() {
                                        chars.next();
                                        if ch == '>' {
                                            break;
                                        }
                                    }
                                    break;
                                }
                            }
                        } else if chars.clone().peek().map_or(false, |&ch| !ch.is_whitespace() && ch != '>') {
                            let mut next_tag = String::new();
                            while let Some(&ch) = chars.peek() {
                                if ch.is_whitespace() || ch == '>' {
                                    break;
                                }
                                next_tag.push(ch);
                                chars.next();
                            }
                            if next_tag.eq_ignore_ascii_case(tag_name) {
                                depth += 1;
                            }
                        }
                    }
                }
            } else {
                result.push(c);
                chars.next();
            }
        } else {
            result.push(c);
            chars.next();
        }
    }
    
    result
}

fn clean_whitespace(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut last_was_space = true;
    
    for c in html.chars() {
        if c.is_whitespace() {
            if !last_was_space {
                result.push(' ');
                last_was_space = true;
            }
        } else {
            result.push(c);
            last_was_space = false;
        }
    }
    
    result.trim().to_string()
}