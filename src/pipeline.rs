use hydrogen_common::models::{CleanedData, RawHtmlData};
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
    let clean_buffer = Arc::new(LockFreeRingBuffer::<CleanedData>::new(100));

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
    let raw_processing_buffer = Arc::clone(&raw_buffer);
    let clean_processing_buffer = Arc::clone(&clean_buffer);
    let processing_handle = task::spawn(async move {
        loop {
            if let Some(html_data) = raw_processing_buffer.pop() {
                let cleaned_data = cleaner::clean_data(html_data).await.unwrap();

                let reduced_data = reduction::reduce(cleaned_data).await.unwrap();

                let transformed_data = transform::transform(reduced_data).await.unwrap();

                if clean_processing_buffer.push(transformed_data).is_err() {
                    eprintln!("Processing buffer is full, dropping transformed data");
                }
            } else {
                sleep(Duration::from_millis(500)).await;
            }
        }
    });

    let sink_buffer = Arc::clone(&clean_buffer);
    let sink_handle = task::spawn(async move {
        loop {
            if let Some(processed_data) = sink_buffer.pop() {
                match sink_data(&processed_data).await {
                    Ok(()) => {
                        eprintln!("DONE, Sinked Data from {}", processed_data.source_url);
                        eprintln!("{:?}", processed_data);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Error processing data: {}", e);
                        eprintln!(" FAILED: Sinking Data from {:?}", processed_data);
                    }
                }
            } else {
                sleep(Duration::from_millis(500)).await;
            }
        }
    });

    let _ = tokio::join!(
        crawler_handle,
        ingester_handle,
        processing_handle,
        sink_handle
    );
}
