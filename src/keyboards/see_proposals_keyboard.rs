use crate::consts::THUMB_UP;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

#[derive(Debug, Clone)]
pub enum SeeProposalsKeyboard {
    ThumbUp,
}

impl SeeProposalsKeyboard {
    pub fn new(text: &str) -> Self {
        match text {
            THUMB_UP => Self::ThumbUp,
            //THUMB_DOWN => Self::ThumbDown,
            _ => Self::ThumbUp,
        }
    }
}

fn see_proposal_keyboard() -> anyhow::Result<InlineKeyboardMarkup> {
    let mut keyboard = InlineKeyboardMarkup::default();
    keyboard = keyboard.append_row(vec![
        InlineKeyboardButton::callback(THUMB_UP, THUMB_UP.to_owned()),
        //InlineKeyboardButton::callback(THUMB_DOWN, THUMB_DOWN.to_owned()),
    ]);
    Ok(keyboard)
}

pub fn new_see_proporsal_keyboard() -> anyhow::Result<InlineKeyboardMarkup> {
    match see_proposal_keyboard() {
        Ok(keyboard) => Ok(keyboard),
        _ => Err(anyhow::anyhow!("Error creating keyboard")),
    }
}
