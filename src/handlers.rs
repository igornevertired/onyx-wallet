use crate::keyboard::create_keyboard;
use teloxide::prelude::*;

pub async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        match text {
            "/start" | "start" | "Main Menu" => {
                bot.send_message(msg.chat.id, "🌟 Welcome to Solana Wallet!")
                    .reply_markup(create_keyboard())
                    .await?;
            }
            "💰 Balance" => {
                bot.send_message(msg.chat.id, "💰 Your balance: 5.25 SOL").await?;
            }
            "📤 Send" => {
                bot.send_message(msg.chat.id, "Enter amount and address:\nExample: `1.5 7ABC...xyz1`")
                    .await?;
            }
            "📥 Receive" => {
                bot.send_message(msg.chat.id, "📥 Your deposit address: 7ABC...xyz1\n\nShare this address to receive SOL")
                    .await?;
            }
            "🆘 Help" => {
                bot.send_message(
                    msg.chat.id,
                    "🆘 Help:\n\n\
                    • Balance - Check your SOL\n\
                    • Send - Transfer SOL\n\
                    • Receive - Get deposit address",
                )
                    .await?;
            }
            _ if text.starts_with('/') => {
                bot.send_message(msg.chat.id, "Please use the menu buttons").await?;
            }
            _ => {
                if let Some((amount, address)) = parse_send_input(text) {
                    bot.send_message(msg.chat.id, format!("🚀 Sent {} SOL to {}", amount, address))
                        .await?;
                } else {
                    bot.send_message(msg.chat.id, "❌ Invalid format. Use buttons or type /help")
                        .await?;
                }
            }
        }
    }
    Ok(())
}

fn parse_send_input(text: &str) -> Option<(String, String)> {
    let mut parts = text.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(amount), Some(address)) => Some((amount.to_string(), address.to_string())),
        _ => None,
    }
}