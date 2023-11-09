use crate::errors::TgError;
use crate::handler::find_keyboard_from_callback;
use crate::keyboards::create_new_proposal_keyboard::new_proporsal_keyboard;
use crate::keyboards::create_new_proposal_keyboard::CreateNewProposalKeyboard;
use crate::keyboards::menu_keyboard;
use crate::messages;
use crate::messages::get_welcome_message;
use crate::storage::TgMessage;
use crate::storage::TgMessageStorage;
use crate::storage::GLOBAL_MAIN_MENU_STORAGE;
use crate::utils::delete_previous_messages;
use std::sync::Arc;
use teloxide::payloads::EditMessageTextSetters;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::InlineKeyboardButtonKind;
use teloxide::types::{CallbackQuery, Message, ParseMode};
use teloxide::Bot;

/// Upon a user clicks the "Main Menu", it'll clear the text and show the menu again
pub async fn handle_menu_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), TgError> {
    let keyboard = menu_keyboard();
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { chat, .. }) = &q.message {
        let welcome_msg = get_welcome_message();

        let message_sent = bot
            .send_message(chat.id, welcome_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
        let message_sent = Arc::new(message_sent);

        // Updates the GLOBAL_STORAGE
        let _user_name = message_sent
            .clone()
            .from()
            .and_then(|user| user.username.as_ref())
            .and_then(|user_name| {
                let message = TgMessage {
                    chat_id: message_sent.chat.id,
                    message_id: message_sent.id,
                    message: message_sent.clone(),
                };
                GLOBAL_MAIN_MENU_STORAGE.insert(user_name.to_string(), message);
                Some(user_name)
            });

        let last_message_id = message_sent.id;
        let _ = delete_previous_messages(bot, chat.id.0, last_message_id.0 - 1, 20).await?;
    };
    Ok(())
}

pub async fn handle_new_proposal_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), TgError> {
    let keyboard = new_proporsal_keyboard(false, false, false, false)?;
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { chat, .. }) = &q.message {
        let proposal_msg = messages::get_new_proposal_message("", "", "", "");

        let _ = bot
            .send_message(chat.id, proposal_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
    };
    Ok(())
}

pub async fn handle_title_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), TgError> {
    if let (Some(button), Some(Message { id, chat, .. })) = (&q.data, &q.message) {
        let proposal_msg = messages::get_new_proposal_message("", "", "", "");

        // Gets current keyboard layout
        let keyboard = find_keyboard_from_callback(q)?.clone();
        let mut new_keyboard = keyboard.clone();
        let button = CreateNewProposalKeyboard::new(button);
        let new_button_text = button.toggle();

        // Change the text to toggled value
        if let Some(button) = new_keyboard
            .inline_keyboard
            .get_mut(1)
            .and_then(|row| row.get_mut(0))
        {
            button.text = new_button_text.to_string();
            button.kind = InlineKeyboardButtonKind::CallbackData(new_button_text.to_string());
        }

        // Edit the message with the new keyboard
        bot.edit_message_text(chat.id, *id, proposal_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(new_keyboard)
            .await?;
    };
    Ok(())
}

pub async fn handle_see_proposals_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), TgError> {
    todo!()
}
