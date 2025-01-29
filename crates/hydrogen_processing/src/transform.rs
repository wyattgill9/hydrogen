use hydrogen_common::models::CleanedData;

pub async fn transform(data: CleanedData) -> Result<CleanedData, Box<dyn std::error::Error>> {
    println!("Transforming data: {}", data.cleaned_html);
    println!("Transformed data URL: {} \n", data.source_url);

    Ok(data)
}
