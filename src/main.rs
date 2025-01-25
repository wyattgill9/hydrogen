use hydrogen_crawler::example::crawler;

// #[tokio::main]
fn main() {
    // url to take html from
    let url = "https://example.com";

    // grab html
    let html: String = crawler(url).unwrap();
    
    println!("{:?}", html);

    // ingest crawler html output

    // parse and serialize html ouput from crawler to a struct

    // preprocess struct data

    // transform struct data

    // sink transformed data


}


