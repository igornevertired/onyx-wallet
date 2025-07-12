use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Show help")]
    Help,
    #[command(description = "Check balance")]
    Balance,
    #[command(description = "Send SOL")]
    Send,
    #[command(description = "Get deposit address")]
    Receive,
}

// Реализация для удобного вывода описаний
impl Command {
    pub fn full_descriptions() -> String {
        Command::full_descriptions().to_string()
    }
}