// use hydrogen_crawler::example::{crawler, crawler_send};
// use hydrogen_ingestion::ingestor::ingest;
// use hydrogen_processing::processor::process;

// // #[tokio::main]
// fn main() {
//     // url to take html from
//     let url = "https://example.com";

//     let crawler_egress = "127.0.0.1:8070";

//     let ingestor_egress = "127.0.0.1:8071";
//     let processor_egress = "127.0.0.1:8072";
//     let transformer_egress = "127.0.0.1:8073";

    
//     let ingestor_ingress = "127.0.0.1:8080";
//     let processor_ingress = "127.0.0.1:8081";
//     let transformer_ingress = "127.0.0.1:8082";
//     let sink_ingress = "127.0.0.1:8083";




//     // grab html
//     let html: String = crawler(url).unwrap();
    
//     println!("{:?}", html);

//     crawler_send(html, );



//     // ingest crawler html output

//     // parse and serialize html ouput from crawler to a struct

//     // preprocess struct data

//     // transform struct data

//     // sink transformed data


// }

