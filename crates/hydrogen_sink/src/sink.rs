use hydrogen_common::models::{CleanedData, RawHtmlData};
use tokio_postgres::{NoTls, Error};
use std::env;
use dotenv::dotenv;

pub async fn sink_data(data: RawHtmlData) -> Result<(), Error> {
    dotenv().ok();

    let cleaned_data = CleanedData::from(data);

    let timestamp = cleaned_data.timestamp as i64;

    println!("Sink data: {:?}", cleaned_data);

    let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    client.execute(
        "INSERT INTO data (source_url, cleaned_html, timestamp) VALUES ($1, $2, $3)",
        &[&cleaned_data.source_url, &cleaned_data.cleaned_html, &(timestamp as i64)]  // CAST the timestamp here
    ).await?;

    Ok(())
}
