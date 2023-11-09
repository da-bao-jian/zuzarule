use crate::consts::{
    CLOSE, DESCRIPTION, EXPIRATION_DATE, MAIN_MENU, STARTING_DATE, SUBMIT_A_PROPOSAL, TITLE,
};
use crate::keyboards::add_emoji;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

#[derive(Debug, Clone)]
pub enum CreateNewProposalKeyboard<'a> {
    MainMenu,
    Close,
    Title(&'a str),
    Description(&'a str),
    StartingDate(&'a str),
    ExpirationDate(&'a str),
    SubmitProposal(&'a str),
}

impl<'a> CreateNewProposalKeyboard<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        match text {
            t if t == MAIN_MENU || t == add_emoji(MAIN_MENU).as_str() => Self::MainMenu,
            t if t == CLOSE || t == add_emoji(CLOSE).as_str() => Self::Close,
            t if t == TITLE || t == add_emoji(TITLE).as_str() => Self::Title(text),
            t if t == DESCRIPTION || t == add_emoji(DESCRIPTION).as_str() => {
                Self::Description(text)
            }
            t if t == STARTING_DATE || t == add_emoji(STARTING_DATE).as_str() => {
                Self::StartingDate(text)
            }
            t if t == EXPIRATION_DATE || t == add_emoji(EXPIRATION_DATE).as_str() => {
                Self::ExpirationDate(text)
            }
            t if t == SUBMIT_A_PROPOSAL || t == add_emoji(SUBMIT_A_PROPOSAL).as_str() => {
                Self::SubmitProposal(text)
            }
            _ => Self::MainMenu,
        }
    }
}

fn create_buy_keyboard(
    title: bool,
    description: bool,
    starting_date: bool,
    expiration_date: bool,
) -> anyhow::Result<InlineKeyboardMarkup> {
    let mut keyboard = InlineKeyboardMarkup::default();

    // 1st row
    keyboard = keyboard.append_row(vec![
        // no need to add emoji in the callback value
        InlineKeyboardButton::callback(add_emoji(MAIN_MENU), MAIN_MENU.to_owned()),
        InlineKeyboardButton::callback(add_emoji(CLOSE), CLOSE.to_owned()),
    ]);

    // 2nd row
    keyboard = keyboard.append_row(vec![match title {
        true => InlineKeyboardButton::callback(add_emoji(TITLE), add_emoji(TITLE)),
        false => InlineKeyboardButton::callback(TITLE.to_owned(), TITLE.to_owned()),
    }]);

    // 3th row
    keyboard = keyboard.append_row(vec![match description {
        true => InlineKeyboardButton::callback(add_emoji(DESCRIPTION), add_emoji(DESCRIPTION)),
        false => InlineKeyboardButton::callback(DESCRIPTION.to_owned(), DESCRIPTION.to_owned()),
    }]);

    // 4th row
    keyboard = keyboard.append_row(vec![match starting_date {
        true => InlineKeyboardButton::callback(add_emoji(STARTING_DATE), add_emoji(STARTING_DATE)),
        false => InlineKeyboardButton::callback(STARTING_DATE.to_owned(), STARTING_DATE.to_owned()),
    }]);

    // 5th row
    keyboard = keyboard.append_row(vec![match expiration_date {
        true => {
            InlineKeyboardButton::callback(add_emoji(EXPIRATION_DATE), add_emoji(EXPIRATION_DATE))
        }
        false => {
            InlineKeyboardButton::callback(EXPIRATION_DATE.to_owned(), EXPIRATION_DATE.to_owned())
        }
    }]);

    // 6th row
    keyboard = keyboard.append_row(vec![match expiration_date {
        true => InlineKeyboardButton::callback(
            add_emoji(SUBMIT_A_PROPOSAL),
            add_emoji(SUBMIT_A_PROPOSAL),
        ),
        false => InlineKeyboardButton::callback(SUBMIT_A_PROPOSAL, SUBMIT_A_PROPOSAL.to_owned()),
    }]);

    Ok(keyboard)
}

pub fn new_proporsal_keyboard(
    title: bool,
    description: bool,
    starting_date: bool,
    expiration_date: bool,
) -> anyhow::Result<InlineKeyboardMarkup> {
    match create_buy_keyboard(title, description, starting_date, expiration_date) {
        Ok(keyboard) => Ok(keyboard),
        _ => Err(anyhow::anyhow!("Error creating keyboard")),
    }
}
