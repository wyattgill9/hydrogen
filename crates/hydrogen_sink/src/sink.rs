use dotenv::dotenv;
use hydrogen_common::models::CleanedData;
use std::env;
use tokio_postgres::{Error, NoTls};

pub async fn sink_data(data: &CleanedData) -> Result<(), Error> {
    dotenv().ok();

    let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    client
        .execute(
            "INSERT INTO data (source_url, cleaned_html, timestamp) VALUES ($1, $2, $3)",
            &[
                &data.source_url,
                &data.cleaned_html,
                &(data.timestamp as i64),
            ],
        )
        .await?;

    Ok(())
}
