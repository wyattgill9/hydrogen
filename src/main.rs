mod pipeline;

#[tokio::main]
async fn main() {
    pipeline::pipeline().await;
}
