use super::dialogue_handlers::DialogueState;
use crate::consts::BOT_NAME;
use crate::errors::TgError;
use crate::keyboards::create_new_proposal_keyboard::new_proporsal_keyboard;
use crate::keyboards::create_new_proposal_keyboard::CreateNewProposalKeyboard;
use crate::keyboards::menu_keyboard;
use crate::keyboards::see_proposals_keyboard::new_see_proporsal_keyboard;
use crate::keyboards::see_proposals_keyboard::SeeProposalsKeyboard;
use crate::messages;
use crate::messages::get_welcome_message;
use crate::storage::Proposal;
use crate::storage::TgMessage;
use crate::storage::TgMessageStorage;
use crate::storage::TgProposalStorage;
use crate::storage::GLOBAL_CREATE_PROPOSAL_STORAGE;
use crate::storage::GLOBAL_MAIN_MENU_STORAGE;
use crate::storage::GLOBAL_PROPOSAL_STORAGE;
use crate::utils::delete_previous_messages;
use regex::Regex;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::dialogue::Storage;
use teloxide::payloads::{EditMessageTextSetters, SendMessageSetters};
use teloxide::prelude::Requester;
use teloxide::types::{CallbackQuery, MediaKind, Message, MessageKind, ParseMode};
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

pub async fn handle_submit_proposal_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), TgError> {
    let keyboard = menu_keyboard();
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { chat, kind, .. }) = &q.message {
        let welcome_msg = get_welcome_message();

        let extract_text = |kind: &MessageKind| -> Option<String> {
            if let MessageKind::Common(common) = kind {
                if let MediaKind::Text(media_text) = &common.media_kind {
                    return Some(media_text.text.clone());
                };
            }
            None
        };
        let text = extract_text(kind).unwrap();

        let extractor = |message: &str, field_name: &str| -> String {
            let pattern = format!(r"{}: ([^\n]*)\n", field_name);
            let re = Regex::new(&pattern.as_str()).unwrap();
            re.captures(message)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
                .unwrap_or_default()
        };

        let title = extractor(&text, "Title");
        let description = extractor(&text, "Description");
        let starting_date = extractor(&text, "Starting Date");
        let expiration_date = extractor(&text, "Expiration Date");
        let proposal = Proposal {
            title,
            description,
            starting_date,
            expiration_date,
            vote: 0,
        };

        let message_sent = bot
            .send_message(chat.id, welcome_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
        let message_sent = Arc::new(message_sent);

        let _user_name = message_sent
            .clone()
            .from()
            .and_then(|user| user.username.as_ref())
            .and_then(|user_name| {
                GLOBAL_PROPOSAL_STORAGE.insert(user_name.to_string(), proposal);
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
        let proposal_msg = messages::get_new_proposal_message();

        let _ = bot
            .send_message(chat.id, proposal_msg)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
    };
    Ok(())
}

pub async fn handle_proposal_fields_callback<'a>(
    bot: &Bot,
    state: DialogueState,
    q: &CallbackQuery,
    storage: Arc<InMemStorage<DialogueState>>,
    callback_type: CreateNewProposalKeyboard<'a>,
) -> Result<(), TgError> {
    bot.answer_callback_query(&q.id).await?;

    if let Some(msg) = &q.message {
        let msg = Arc::new(msg);
        let _user_name = msg
            .clone()
            .from()
            .and_then(|user| user.username.as_ref())
            .and_then(|user_name| {
                let message = TgMessage {
                    chat_id: msg.chat.id,
                    message_id: msg.id,
                    message: (*msg).clone().into(),
                };
                GLOBAL_CREATE_PROPOSAL_STORAGE.insert(user_name.to_string(), message);
                Some(user_name)
            });
    }

    if let Some(Message { chat, .. }) = &q.message {
        storage.clone().update_dialogue(chat.id, state).await?;
        match callback_type {
            CreateNewProposalKeyboard::Title(_) => {
                bot.send_message(chat.id, "Enter the proposal Title")
                    .await?;
                storage
                    .update_dialogue(chat.id, DialogueState::TitleReceived)
                    .await?;
            }
            CreateNewProposalKeyboard::Description(_) => {
                bot.send_message(chat.id, "Enter the proposal Description")
                    .await?;
                storage
                    .update_dialogue(chat.id, DialogueState::DescriptionReceived)
                    .await?;
            }
            CreateNewProposalKeyboard::StartingDate(_) => {
                bot.send_message(chat.id, "Enter the proposal Starting Date")
                    .await?;
                storage
                    .update_dialogue(chat.id, DialogueState::StartingDateReceived)
                    .await?;
            }
            CreateNewProposalKeyboard::ExpirationDate(_) => {
                bot.send_message(chat.id, "Enter the proposal Expiration Date")
                    .await?;
                storage
                    .update_dialogue(chat.id, DialogueState::ExpirationDateReceived)
                    .await?;
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn handle_see_proposals_callback(bot: &Bot, q: &CallbackQuery) -> Result<(), TgError> {
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { chat, .. }) = &q.message {
        if let Some(proposals) = GLOBAL_PROPOSAL_STORAGE.get(BOT_NAME.to_string()) {
            for proposal in proposals {
                let keyboard = new_see_proporsal_keyboard()?;
                let msg = format!(
                    "Title: {}\nDescription: {}\nStarting Date: {}\nExpiration Date: {}\nVotes: {}",
                    proposal.title,
                    proposal.description,
                    proposal.starting_date,
                    proposal.expiration_date,
                    proposal.vote
                );

                let _message_sent = bot
                    .send_message(chat.id, msg)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(keyboard)
                    .await?;
            }
        }
    };
    Ok(())
}

pub async fn handle_thumb_up_callback(
    bot: &Bot,
    q: &CallbackQuery,
    _propoal_type: SeeProposalsKeyboard,
) -> Result<(), TgError> {
    bot.answer_callback_query(&q.id).await?;
    if let Some(Message { kind, chat, id, .. }) = &q.message {
        let extract_text = |kind: &MessageKind| -> Option<String> {
            if let MessageKind::Common(common) = kind {
                if let MediaKind::Text(media_text) = &common.media_kind {
                    return Some(media_text.text.clone());
                }
            }
            None
        };

        let mut new_message: String = "".to_string();
        let text = extract_text(kind).unwrap();
        let re = Regex::new(r"Votes: (\d+)").unwrap();
        if let Some(caps) = re.captures(&text) {
            if let Some(match_) = caps.get(1) {
                if let Ok(current_vote) = match_.as_str().parse::<i32>() {
                    let incremented_vote = current_vote + 1;
                    new_message = re
                        .replace(&text, format!("Votes: {}", incremented_vote))
                        .to_string();
                }
            }
        }

        let keyboard = new_see_proporsal_keyboard()?;
        bot.edit_message_text(chat.id, *id, new_message)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;
    }
    Ok(())
}
