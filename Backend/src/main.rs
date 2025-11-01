mod map;
mod io;
mod types;

use std::fs::File;
use tokio::io::stdin;
use crate::map::viz_places::viz_map;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::io::client::handle_client;
use crate::map::gen_places::fetch_map;
mod chatbots;
use chatbots::types::ChatManager;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    /*let map = {
        if false {
            let map = fetch_map("Durham", 5, 100.0).await.unwrap();
            viz_map(&map);
            let file = File::create("map.json")?;
            serde_json::to_writer_pretty(file, &map)?;
            println!("✅ Map saved to map.json");
            map
        } else {
            let file = File::open("map.json")?;
            serde_json::from_reader(file)?
        }
    };*/

    init_connection().await;
    Ok(())
}


async fn chat() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY in a .env file or environment.");

    let mut manager = ChatManager::new("gpt-5-nano");

    manager.add_character("Merlin", "wise old wizard");
    manager.add_character("Rex", "sarcastic robot");
    manager.add_character("Luna", "mysterious poet");

    manager.switch_to("Merlin");

    println!("Type messages. Use /switch <name> to change character. /exit to quit.\n");

    let stdin = BufReader::new(stdin());
    let mut lines = stdin.lines();

    while let Some(line) = lines.next_line().await? {
        let input = line.trim();

        if input.starts_with("/switch ") {
            let target = input.strip_prefix("/switch ").unwrap().trim();
            manager.switch_to(target);
            continue;
        }

        if input == "/exit" {
            break;
        }

        manager.send_message(input).await;
    }

    Ok(())
}


pub async fn init_connection() {
    let port = "9999";
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Failed to bind to port");

    println!("Server listening on localhost:{}", port);


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