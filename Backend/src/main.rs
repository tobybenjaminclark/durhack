mod map;
mod io;
mod types;

use crate::map::viz_places::viz_map;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::io::client::handle_client;
use crate::map::gen_places::fetch_map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_connection().await;
    Ok(())
}


pub async fn init_connection() {
    let port = "9999";
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Failed to bind to port");

    println!("Server listening on localhost:{}", port);

    let map = fetch_map("Nottingham", 5, 100.0).await.unwrap();
    viz_map(&map);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New connection from {}", addr);
                tokio::spawn(handle_client(stream));
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}