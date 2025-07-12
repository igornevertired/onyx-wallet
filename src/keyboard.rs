use teloxide::types::{KeyboardButton, ReplyMarkup};

pub fn create_keyboard() -> ReplyMarkup {
    let buttons = vec![
        vec![KeyboardButton::new("💰 Balance")],
        vec![KeyboardButton::new("📤 Send"), KeyboardButton::new("📥 Receive")],
        vec![KeyboardButton::new("🆘 Help")],
    ];

    ReplyMarkup::keyboard(buttons)
}