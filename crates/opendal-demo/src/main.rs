use std::time::Duration;
use opendal::Result;
use opendal::layers::LoggingLayer;
use opendal::services;
use opendal::Operator;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // Pick a builder and configure it.
    let mut builder = services::S3::default();
    builder.bucket("test");
    builder.endpoint("http://127.0.0.1:9000");
    builder.access_key_id("minioadmin");
    builder.secret_access_key("minioadmin");

    // Init an operator
    let op = Operator::new(builder)?
        // Init with logging layer enabled.
        .layer(LoggingLayer::default())
        .finish();

    // Write data
    op.write("hello.txt", "Hello, World!").await?;

    // Read data
    let bs = op.read("hello.txt").await?;

    // Fetch metadata
    let meta = op.stat("hello.txt").await?;
    let mode = meta.mode();
    let length = meta.content_length();

    // Delete
    sleep(Duration::from_secs(30)).await;
    op.delete("hello.txt").await?;

    Ok(())
}