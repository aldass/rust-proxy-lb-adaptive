use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(
    mut inbound: TcpStream,
    proxy_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        tokio::io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        tokio::io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listen_addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let proxy_addr = "127.0.0.1:8081".parse::<SocketAddr>()?;

    let listener = TcpListener::bind(listen_addr).await?;
    println!("Listening on: {}", listen_addr);

    loop {
        let (socket, _) = listener.accept().await?;
        let proxy_addr = proxy_addr.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, proxy_addr).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}
