use crate::consts::{BOT_NAME, DESCRIPTION, EXPIRATION_DATE, STARTING_DATE, TITLE};
use crate::handler::delete_up_to_messages;
use crate::keyboards::add_emoji;
use crate::messages::parse_message;
use crate::storage::{TgMessage, TgMessageStorage, GLOBAL_CREATE_PROPOSAL_STORAGE};
use crate::TgError;
use teloxide::types::MessageKind;
use teloxide::{
    dispatching::dialogue::{Dialogue, InMemStorage},
    payloads::EditMessageTextSetters,
    requests::Requester,
    types::{InlineKeyboardButtonKind, MediaKind, Message, ParseMode},
    Bot,
};

use super::find_keyboard_from_message;

pub type ProposalPromptDialogue = Dialogue<DialogueState, InMemStorage<DialogueState>>;

/// Dialogue state
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub enum DialogueState {
    #[default]
    StartTitlePrompt,
    TitleReceived,
    DescriptionReceived,
    StartingDateReceived,
    ExpirationDateReceived,
}

pub async fn start_title_dialogue_handler(
    bot: Bot,
    dialogue: ProposalPromptDialogue,
    msg: Message,
) -> Result<(), TgError> {
    bot.send_message(msg.chat.id, "Enter the title").await?;
    dialogue.update(DialogueState::StartTitlePrompt).await?;
    Ok(())
}

pub async fn receive_title_handler(
    bot: Bot,
    dialogue: ProposalPromptDialogue,
    msg: Message,
) -> Result<(), TgError> {
    let text = match msg.text() {
        Some(t) => t,
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
            return Ok(());
        }
    };

    if let Some(menu) = GLOBAL_CREATE_PROPOSAL_STORAGE.get(BOT_NAME.to_string()) {
        let extract_text = |tg_message: &TgMessage| -> Option<String> {
            if let MessageKind::Common(common) = &tg_message.message.kind {
                if let MediaKind::Text(media_text) = &common.media_kind {
                    return Some(media_text.text.clone());
                }
            }
            None
        };

        let extracted_text = extract_text(&menu).unwrap();
        let proposal_msg = parse_message(extracted_text.as_str(), Some(text), None, None, None);
        let menu_msg = menu.message;
        let msg_id = menu.message_id;
        let keyboard = find_keyboard_from_message(&menu_msg)?;
        let mut new_keyboard = keyboard.clone();

        let new_button_text = add_emoji(TITLE);

        if let Some(button) = new_keyboard
            .inline_keyboard
            .get_mut(1)
            .and_then(|row| row.get_mut(0))
        {
            button.text = new_button_text.to_string();
            button.kind = InlineKeyboardButtonKind::CallbackData(new_button_text.to_string());
        }

        // Edit the message with the new keyboard
        bot.edit_message_text(msg.chat.id, msg_id, proposal_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(new_keyboard)
            .await?;
        dialogue.exit().await?;

        let _ = delete_up_to_messages(&bot, msg.chat.id.0, msg.id.0, msg_id.0).await?;
    } else {
        log::warn!("message not found");
    }
    Ok(())
}

pub async fn receive_description_handler(
    bot: Bot,
    dialogue: ProposalPromptDialogue,
    msg: Message,
) -> Result<(), TgError> {
    let text = match msg.text() {
        Some(t) => t,
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
            return Ok(());
        }
    };

    if let Some(menu) = GLOBAL_CREATE_PROPOSAL_STORAGE.get(BOT_NAME.to_string()) {
        let extract_text = |tg_message: &TgMessage| -> Option<String> {
            if let MessageKind::Common(common) = &tg_message.message.kind {
                if let MediaKind::Text(media_text) = &common.media_kind {
                    return Some(media_text.text.clone());
                }
            }
            None
        };

        let extracted_text = extract_text(&menu).unwrap();
        let proposal_msg = parse_message(extracted_text.as_str(), None, Some(text), None, None);
        let menu_msg = menu.message;
        let msg_id = menu.message_id;
        let keyboard = find_keyboard_from_message(&menu_msg)?;
        let mut new_keyboard = keyboard.clone();

        let new_button_text = add_emoji(DESCRIPTION);

        if let Some(button) = new_keyboard
            .inline_keyboard
            .get_mut(2)
            .and_then(|row| row.get_mut(0))
        {
            button.text = new_button_text.to_string();
            button.kind = InlineKeyboardButtonKind::CallbackData(new_button_text.to_string());
        }

        // Edit the message with the new keyboard
        bot.edit_message_text(msg.chat.id, msg_id, proposal_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(new_keyboard)
            .await?;
        dialogue.exit().await?;

        let _ = delete_up_to_messages(&bot, msg.chat.id.0, msg.id.0, msg_id.0).await?;
    } else {
        log::warn!("message not found");
    }
    Ok(())
}

pub async fn receive_starting_date_handler(
    bot: Bot,
    dialogue: ProposalPromptDialogue,
    msg: Message,
) -> Result<(), TgError> {
    let text = match msg.text() {
        Some(t) => t,
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
            return Ok(());
        }
    };

    if let Some(menu) = GLOBAL_CREATE_PROPOSAL_STORAGE.get(BOT_NAME.to_string()) {
        let extract_text = |tg_message: &TgMessage| -> Option<String> {
            if let MessageKind::Common(common) = &tg_message.message.kind {
                if let MediaKind::Text(media_text) = &common.media_kind {
                    return Some(media_text.text.clone());
                }
            }
            None
        };

        let extracted_text = extract_text(&menu).unwrap();
        let proposal_msg = parse_message(extracted_text.as_str(), None, None, Some(text), None);
        let menu_msg = menu.message;
        let msg_id = menu.message_id;
        let keyboard = find_keyboard_from_message(&menu_msg)?;
        let mut new_keyboard = keyboard.clone();

        let new_button_text = add_emoji(STARTING_DATE);

        if let Some(button) = new_keyboard
            .inline_keyboard
            .get_mut(3)
            .and_then(|row| row.get_mut(0))
        {
            button.text = new_button_text.to_string();
            button.kind = InlineKeyboardButtonKind::CallbackData(new_button_text.to_string());
        }

        // Edit the message with the new keyboard
        bot.edit_message_text(msg.chat.id, msg_id, proposal_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(new_keyboard)
            .await?;
        dialogue.exit().await?;

        let _ = delete_up_to_messages(&bot, msg.chat.id.0, msg.id.0, msg_id.0).await?;
    } else {
        log::warn!("message not found");
    }
    Ok(())
}

pub async fn receive_expiration_date_handler(
    bot: Bot,
    dialogue: ProposalPromptDialogue,
    msg: Message,
) -> Result<(), TgError> {
    let text = match msg.text() {
        Some(t) => t,
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
            return Ok(());
        }
    };

    if let Some(menu) = GLOBAL_CREATE_PROPOSAL_STORAGE.get(BOT_NAME.to_string()) {
        let extract_text = |tg_message: &TgMessage| -> Option<String> {
            if let MessageKind::Common(common) = &tg_message.message.kind {
                if let MediaKind::Text(media_text) = &common.media_kind {
                    return Some(media_text.text.clone());
                }
            }
            None
        };

        let extracted_text = extract_text(&menu).unwrap();
        let proposal_msg = parse_message(extracted_text.as_str(), None, None, None, Some(text));
        let menu_msg = menu.message;
        let msg_id = menu.message_id;
        let keyboard = find_keyboard_from_message(&menu_msg)?;
        let mut new_keyboard = keyboard.clone();

        let new_button_text = add_emoji(EXPIRATION_DATE);

        if let Some(button) = new_keyboard
            .inline_keyboard
            .get_mut(4)
            .and_then(|row| row.get_mut(0))
        {
            button.text = new_button_text.to_string();
            button.kind = InlineKeyboardButtonKind::CallbackData(new_button_text.to_string());
        }

        // Edit the message with the new keyboard
        bot.edit_message_text(msg.chat.id, msg_id, proposal_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(new_keyboard)
            .await?;
        dialogue.exit().await?;

        let _ = delete_up_to_messages(&bot, msg.chat.id.0, msg.id.0, msg_id.0).await?;
    } else {
        log::warn!("message not found");
    }
    Ok(())
}
