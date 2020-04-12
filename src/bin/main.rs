use std::io;
use tokio::net::{TcpListener, TcpStream};

async fn process_socket(socket: TcpStream) {
    log::info!("process socket!");
}

#[tokio::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "log");
    env_logger::init();

    let mut listener = TcpListener::bind("127.0.0.1:1883").await?;
    loop {
        match listener.accept().await {
            Ok((socket, _)) => process_socket(socket).await,
            Err(err) => log::error!("Failed to accept incoming connection: {:?}", err),
        }
    }
}
