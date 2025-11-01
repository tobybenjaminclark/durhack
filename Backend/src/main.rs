use tokio::io;
use tokio::io::AsyncBufReadExt;

mod chatbots;
use chatbots::llm::*;
use chatbots::types::ChatManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY in a .env file or environment.");

    let mut manager = ChatManager::new("gpt-5-nano");

    manager.add_character("Merlin", "wise old wizard");
    manager.add_character("Rex", "sarcastic robot");
    manager.add_character("Luna", "mysterious poet");

    manager.switch_to("Merlin");

    println!("Type messages. Use /switch <name> to change character. /exit to quit.\n");

    let stdin = io::BufReader::new(io::stdin());
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
