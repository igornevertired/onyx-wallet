use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Initialize logger
    pretty_env_logger::init();

    if let Err(e) = dotenv() {
        log::error!("Failed to load .env file: {}", e);
        panic!("Please create a .env file with TELOXIDE_TOKEN");
    }

    // Check for token
    let token = env::var("TELOXIDE_TOKEN").unwrap_or_else(|_| {
        log::error!("TELOXIDE_TOKEN not found in environment variables");
        panic!("Please set TELOXIDE_TOKEN in .env file");
    });

    log::info!("Starting Solana Telegram Wallet...");

    let bot = Bot::new(token);

    let handler = |bot: Bot, msg: Message| async move {
        let response = match msg.text() {
            Some("/start") => "🌟 Welcome to Solana Telegram Wallet!\n\n\
                               A simple and convenient way to send and receive SOL\n\
                               directly in Telegram. No complicated interfaces!\n\n\
                               Available commands:\n\
                               /balance - Check your SOL balance\n\
                               /send [amount] [address] - Send SOL\n\
                               /receive - Get your deposit address\n\
                               /help - Show this message",

            Some("/help") => "🆘 Help Center:\n\n\
                             This bot allows you to:\n\
                             - Check your Solana balance\n\
                             - Send SOL to other wallets\n\
                             - Receive SOL via your personal address\n\n\
                             All transactions are secure and on-chain.",

            Some(text) if text.starts_with("/send") => "🚀 Transaction processing... (This would send SOL in real implementation)",

            Some("/balance") => "💰 Your balance: 5.25 SOL (example)",

            Some("/receive") => "📥 Your deposit address: 7ABC...xyz1 (example)\n\n\
                                Share this address to receive SOL payments",

            Some(_) => "❌ Unknown command. Type /help for available commands",

            None => "🔍 Please send text commands only",
        };

        bot.send_message(msg.chat.id, response).await?;
        Ok(())
    };

    teloxide::repl(bot, handler).await;
}