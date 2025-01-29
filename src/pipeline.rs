use hydrogen_common::models::RawHtmlData;
use hydrogen_common::ring_buffer::LockFreeRingBuffer;
use hydrogen_crawler::example::crawler;
use hydrogen_ingestion::ingestor::ingest_data;
use hydrogen_processing::*;
use hydrogen_sink::sink::sink_data;

use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tokio::{self, task};

pub async fn pipeline() {
    let raw_buffer = Arc::new(LockFreeRingBuffer::<RawHtmlData>::new(100));

    let crawler_buffer = Arc::clone(&raw_buffer);
    let crawler_handle = task::spawn(async move {
        let url = "https://example.com";
        match crawler(url).await {
            Ok(raw_data) => {
                if crawler_buffer.push(raw_data).is_err() {
                    eprintln!("Crawler buffer is full, dropping data");
                }
            }
            Err(e) => eprintln!("Crawl error: {}", e),
        }
    });

    let ingester_buffer = Arc::clone(&raw_buffer);
    let ingester_handle = task::spawn(async move {
        loop {
            if let Some(raw_data) = ingester_buffer.pop() {
                let ingested_data = ingest_data(raw_data);
                if ingester_buffer.push(ingested_data.unwrap()).is_err() {
                    eprintln!("Ingester buffer is full, dropping processed data");
                }
            } else {
                sleep(Duration::from_millis(500)).await;
            }
        }
    });

    // Processing
    let processing_buffer = Arc::clone(&raw_buffer);
    let processing_handle = task::spawn(async move {
        loop {
            if let Some(html_data) = processing_buffer.pop() {
                let cleaned_data = cleaner::clean_data(html_data).await.unwrap();
                println!("Cleaned HTML");

                let reduced_data = reduction::reduce(cleaned_data).await.unwrap();
                println!("Reduced HTML");

                match transform::transform(reduced_data).await {
                    Ok(transformed_data) => {
                        let transformed_raw_data: RawHtmlData = transformed_data.into(); // Convert CleanedData to RawHtmlData
                        println!("Successfully Transformed data");

                        if processing_buffer.push(transformed_raw_data.into()).is_err() {
                            eprintln!("Processing buffer is full");
                        }

                        break;
                    }
                    Err(e) => eprintln!("Processing error: {}", e),
                }
            } else {
                sleep(Duration::from_millis(500)).await;
            }
        }
    });

    let sink_buffer = Arc::clone(&raw_buffer);
    let sink_handle = task::spawn(async move {
        loop {
            if let Some(transformed_raw_data) = sink_buffer.pop() {  // Pop returns transformed_raw_data, not raw_data
                match sink_data(transformed_raw_data).await {
                    Ok(()) => {
                        eprintln!("DONE, Sinked data from www.example.com");
                        break;
                    },
                    Err(e) => {
                        eprintln!("Error processing data: {}", e);
                    }
                }
            } else {
                sleep(Duration::from_millis(500)).await;
            }
        }
    });

    let _ = tokio::join!(crawler_handle, ingester_handle, processing_handle, sink_handle);
}
