use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub async fn handle_client(
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

pub async fn start_proxy(
    listen_addr: SocketAddr,
    backends: Vec<SocketAddr>,
) -> Result<(), Box<dyn std::error::Error>> {
    let backends = Arc::new(Mutex::new(VecDeque::from(backends)));

    let listener = TcpListener::bind(listen_addr).await?;
    println!("Listening on: {}", listen_addr);

    loop {
        let (socket, _) = listener.accept().await?;
        let backends = Arc::clone(&backends);

        tokio::spawn(async move {
            let proxy_addr = {
                let mut backends = backends.lock().await;
                let addr = backends.pop_front().unwrap();
                backends.push_back(addr);
                addr
            };

            if let Err(e) = handle_client(socket, proxy_addr).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_handle_client() {
        let proxy_addr = "127.0.0.1:8082".parse::<SocketAddr>().unwrap();
        let listener = TcpListener::bind(proxy_addr).await.unwrap();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = [0; 4];
            socket.read_exact(&mut buf).await.unwrap();
            assert_eq!(&buf, b"ping");
            socket.write_all(b"pong").await.unwrap();
        });

        let client_addr = "127.0.0.1:8083".parse::<SocketAddr>().unwrap();
        let listener = TcpListener::bind(client_addr).await.unwrap();

        tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            handle_client(socket, proxy_addr).await.unwrap();
        });

        let mut client = TcpStream::connect(client_addr).await.unwrap();
        client.write_all(b"ping").await.unwrap();
        let mut buf = [0; 4];
        client.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"pong");
    }
}
