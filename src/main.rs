use futures_util::StreamExt;
use reqwest::Client;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let total_bytes = Arc::new(AtomicU64::new(0));
    let client = Arc::new(
        Client::builder()
            .tcp_keepalive(Duration::from_secs(60))
            .pool_max_idle_per_host(100)
            .build()
            .unwrap(),
    );

    let urls = vec![
        "http://ipv4.download.thinkbroadband.com/1GB.zip",
        "http://ipv4.download.thinkbroadband.com/512MB.zip",
    ];

    let parallel_downloads = 50;
    
    let stats_counter = Arc::clone(&total_bytes);
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(2)).await;
            let bytes = stats_counter.load(Ordering::Relaxed);
            let gb = bytes as f64 / 1_073_741_824.0;
            let mb = bytes as f64 / 1_048_576.0;
            println!("📊 Total Consumed: {:.2} GB ({:.0} MB)", gb, mb);
        }
    });

    println!("🚀 Internet Hogger Active...");

    let url_stream = futures_util::stream::iter(urls.iter().cycle());

    url_stream
        .for_each_concurrent(parallel_downloads, |url| {
            let client = Arc::clone(&client);
            let counter = Arc::clone(&total_bytes);
            async move {
                let _ = hog_bandwidth(&client, url, counter).await;
            }
        })
        .await;
}

async fn hog_bandwidth(client: &Client, url: &str, counter: Arc<AtomicU64>) -> reqwest::Result<()> {
    let mut resp = client.get(url).send().await?;
    
    while let Some(chunk) = resp.chunk().await? {
        counter.fetch_add(chunk.len() as u64, Ordering::Relaxed);
    }

    Ok(())
}
