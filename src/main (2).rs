use teloxide::prelude::*;
use funkytextgenbot::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting funkytextgenbot...");

    let bot = Bot::from_env().auto_send();

    teloxide::dialogues_repl(bot, |message, dialogue| async move {
            handle_message(message, dialogue).await.expect("Something is wrong with the bot!")
        })
        .await;
}