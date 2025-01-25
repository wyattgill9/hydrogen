// use tokio::net::TcpListener;
// use tokio::io::AsyncReadExt;
// use std::error::Error;

// pub async fn ingest(addr: &str) -> Result<(), Box<dyn Error>> {
//     let listener = TcpListener::bind(addr).await?;
//     println!("Ingestion service running on {}", addr);

//     loop {
//         let (mut socket, peer_addr) = listener.accept().await?;
//         println!("Accepted connection from {}", peer_addr);

//         let mut buffer = vec![0; 8192];
//         let bytes_read = socket.read(&mut buffer).await?;

//         if bytes_read == 0 {
//             println!("Empty data received, skipping...");
//             continue;
//         }

//         let raw_html = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

//         parse_html(raw_html, peer_addr.to_string()).await;

//         println!("Processing HTML from {} at {}", html_data.source_url, html_data.timestamp);
//     }
// }
