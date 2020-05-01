use std::error::Error;
use std::io::Write;
use std::net::{ToSocketAddrs, Ipv4Addr};
use std::net::SocketAddrV4;

use native_tls::{Certificate, TlsConnector};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn blog() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = "blog.biofan.org:443"
        .to_socket_addrs()?
        .next()
        .ok_or("failed to resolve biofan.org")?;

    let socket = TcpStream::connect(&addr).await?;
    let connector = TlsConnector::builder().build()?;
    let connector = tokio_tls::TlsConnector::from(connector);
    let mut socket = connector.connect("biofan.org", socket).await?;

    let _ = socket.write_all(b"GET / HTTP/1.0\r\nHost: biofan.org\r\n\r\n").await?;
    let mut buf = Vec::new();
    socket.read_to_end(&mut buf).await?;

    let _ = std::io::stdout().write_all(&buf);
    Ok(())
}

async fn mqtt() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = "127.0.0.1:8883"
        .to_socket_addrs()?
        .next()
        .ok_or("failed to r esolve localhost")?;
    println!("addr: {:?}", addr);
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let addr = SocketAddrV4::new(localhost, 8883);

    let ca_root_buf = include_bytes!("root-ca.pem");
    let root_ca = Certificate::from_pem(ca_root_buf.as_ref())?;
    let mut tls_builder = TlsConnector::builder();
    tls_builder.add_root_certificate(root_ca);
    let connector = tls_builder.build()?;
    let connector = tokio_tls::TlsConnector::from(connector);

    let socket = TcpStream::connect(&addr).await?;
    // let mut socket = connector.connect("example.org", socket).await?;
    let mut socket = connector.connect("localhost", socket).await?;
    let _ = socket.write_all(b"GET / HTTP/1.0\r\nHost: biofan.org\r\n\r\n").await?;
    let mut buf = Vec::new();
    socket.read_to_end(&mut buf).await?;

    let _ = std::io::stdout().write_all(&buf);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    blog().await?;
    mqtt().await
}