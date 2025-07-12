mod keyboard;
mod handlers;

use handlers::handle_message;
use teloxide::prelude::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let bot = Bot::from_env();
    log::info!("Starting Solana Wallet Bot...");

    teloxide::repl(bot, handle_message).await;
}