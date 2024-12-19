use rust_proxy_lb_adaptive::start_proxy;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listen_addr = "0.0.0.0:8088".parse::<SocketAddr>()?;

    // TODO: Use a more robust dynamic configuration system
    let backend1_addr = env::var("BACKEND1_ADDR")?;
    let backend2_addr = env::var("BACKEND2_ADDR")?;
    let backend3_addr = env::var("BACKEND3_ADDR")?;

    println!(
        "Backend server addresses: {}, {}, {}",
        backend1_addr, backend2_addr, backend3_addr
    );

    let backend_addrs = vec![
        backend1_addr.parse::<SocketAddr>().map_err(|e| {
            eprintln!("Failed to parse BACKEND1_ADDR: {}", e);
            e
        })?,
        backend2_addr.parse::<SocketAddr>().map_err(|e| {
            eprintln!("Failed to parse BACKEND2_ADDR: {}", e);
            e
        })?,
        backend3_addr.parse::<SocketAddr>().map_err(|e| {
            eprintln!("Failed to parse BACKEND3_ADDR: {}", e);
            e
        })?,
    ];

    start_proxy(listen_addr, backend_addrs).await
}
