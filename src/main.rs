use futures_util::stream::StreamExt;
use reqwest::Client;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let client = Arc::new(Client::new());

    let urls = vec![
        "http://ipv4.download.thinkbroadband.com/100MB.zip",
        "http://ipv4.download.thinkbroadband.com/512MB.zip",
        "http://ipv4.download.thinkbroadband.com/1GB.zip",
    ];

    let concurrency = 10;

    loop {
        let mut handles = vec![];

        for url in &urls {
            for _ in 0..concurrency {
                let url = url.to_string();
                let client = client.clone();

                let handle = tokio::spawn(async move {
                    let resp = client.get(&url).send().await;
                    match resp {
                        Ok(mut r) => {
                            let mut stream = r.bytes_stream();
                            let mut total: usize = 0;
                            while let Some(chunk) = stream.next().await {
                                if let Ok(bytes) = chunk {
                                    total += bytes.len();
                                }
                            }
                            println!("Downloaded {} bytes from {}", total, url);
                        }
                        Err(e) => {
                            eprintln!("Error downloading {}: {}", url, e);
                        }
                    }
                });
                handles.push(handle);
            }
        }

        futures_util::future::join_all(handles).await;

        sleep(Duration::from_millis(100)).await;
    }
}
