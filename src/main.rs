// use hydrogen_crawler::example::crawler;

// #[tokio::main]
// fn main() {
// url to take html from
// let url = "https://example.com";

// grab html
// let html: String = crawler(url).unwrap();

// println!("{:?}", html);

// ingest crawler html output

// parse and serialize html ouput from crawler to a struct

// preprocess struct data

// transform struct data

// sink transformed data

//}

use hydrogen_common::models::{CleanedData, RawHtmlData};
use hydrogen_common::ring_buffer::LockFreeRingBuffer;
use hydrogen_crawler::example::crawler;
use hydrogen_ingestion::ingestor::ingest_data;
use hydrogen_processing::*;

use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tokio::{self, task};

#[tokio::main]
async fn main() {
    let raw_buffer = Arc::new(LockFreeRingBuffer::new(100));

    // Crawler (Producer)
    let crawler_buffer = Arc::clone(&raw_buffer);
    let crawler_handle = task::spawn(async move {
        let url = "https://example.com";
        match crawler(url).await {
            Ok(raw_data) => {
                if crawler_buffer.push(raw_data).is_err() {
                    eprintln!("Buffer is full, dropping data");
                }
            }
            Err(e) => eprintln!("Crawl error: {}", e),
        }
    });

    // Ingester (Consumer)
    let ingester_buffer = Arc::clone(&raw_buffer);
    let ingester_handle = task::spawn(async move {
        loop {
            if let Some(raw_data) = ingester_buffer.pop() {
                let ingested_data = ingest_data(raw_data);
                if ingester_buffer.push(ingested_data.unwrap()).is_err() {
                    eprintln!("Buffer is full, dropping processed data");
                }
            } else {
                sleep(Duration::from_millis(500)).await;
            }
        }
    });

    // Processing
    let processing_buffer = Arc::clone(&raw_buffer);
    let processing_handle = task::spawn(async move {
        println!("Processing data...");
        loop {
            if let Some(html_data) = processing_buffer.pop() {
                match cleaner::clean_data(html_data).await {
                    Ok(CleanedData) => println!("Successfully cleaned data"),
                    Err(e) => eprintln!("Processing error: {}", e),
                }
            } else {
                sleep(Duration::from_millis(500)).await;
            }
        }
    });

    let _ = tokio::join!(crawler_handle, ingester_handle, processing_handle);
}
