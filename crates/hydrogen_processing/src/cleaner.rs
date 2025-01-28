use hydrogen_common::models::{CleanedData, RawHtmlData};

pub async fn clean_data(raw_data: RawHtmlData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    let mut cleaned_html = raw_data.raw_html;

    cleaned_html = remove_comments(&cleaned_html);

    cleaned_html = remove_tags(&cleaned_html);

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
        if c == '<'
            && chars.clone().nth(1) == Some('!')
            && chars.clone().nth(2) == Some('-')
            && chars.clone().nth(3) == Some('-')
        {
            while let Some(ch) = chars.next() {
                if ch == '-'
                    && chars.clone().peek() == Some(&'-')
                    && chars.clone().nth(1) == Some('>')
                {
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

fn remove_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut inside_tag = false;

    for c in html.chars() {
        match c {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ => {
                if !inside_tag {
                    result.push(c);
                }
            }
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
