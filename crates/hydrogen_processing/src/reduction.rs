use hydrogen_common::models::CleanedData;

// commmon words to reduce
const FILLER_WORDS: [&str; 19] = [
    "a", "an", "and", "are", "as", "at", "be", "by", "for", "in", "is", "it", "of", "on", "that",
    "the", "this", "to", "with",
];

pub async fn reduce(mut data: CleanedData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    let text = data.cleaned_html.to_lowercase();

    let words: Vec<&str> = text.split_whitespace().collect();

    let reduced_words: Vec<&str> = words
        .into_iter()
        .filter(|word| {
            !FILLER_WORDS.contains(word) && word.len() >= 3 && !word.chars().all(|c| c.is_numeric())
        })
        .collect();

    let mut unique_words = Vec::new();
    for word in reduced_words {
        if !unique_words.contains(&word) {
            unique_words.push(word);
        }
    }

    data.cleaned_html = unique_words.join(" ");

    Ok(data)
}
