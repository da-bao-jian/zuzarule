pub mod create_new_proposal_keyboard;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::consts::{
    CLOSE, DESCRIPTION, EXPIRATION_DATE, MAIN_MENU, STARTING_DATE, SUBMIT_PROPOSAL, TITLE,
};

/// Default layout for the keyboard
fn create_keyboard(actions: Vec<&str>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for action in actions.chunks(3) {
        let row = action
            .iter()
            .map(|&action| InlineKeyboardButton::callback(action.to_owned(), action.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn add_emoji(text: &str) -> String {
    let button = match text {
        MAIN_MENU => format!("🏠 {}", text),
        CLOSE => format!("❌ {}", text),
        TITLE => format!("✅ {}", text),
        DESCRIPTION => format!("✅ {}", text),
        STARTING_DATE => format!("✅ {}", text),
        EXPIRATION_DATE => format!("✅{}", text),
        SUBMIT_PROPOSAL => format!("✅{}", text),
        _ => text.to_string(),
    };
    button
}

pub fn menu_keyboard() -> InlineKeyboardMarkup {
    create_keyboard(vec!["See Proposals", "Create a Proposal"])
}
