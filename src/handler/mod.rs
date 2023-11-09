pub mod callback_handlers;
pub mod dialogue_handlers;

use crate::consts::SUBMIT_A_PROPOSAL;
use crate::TgError;
use teloxide::types::{CallbackQuery, InlineKeyboardMarkup, Message};
use teloxide::{
    prelude::Requester,
    types::{ChatId, MessageId},
    Bot,
};
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub enum SubMenuType {
    CreateNewProposal,
    SeeProposals,
}

pub fn match_sub_menu(q: &CallbackQuery) -> Option<SubMenuType> {
    let res = q
        .message
        .as_ref()
        .and_then(|msg| msg.reply_markup())
        .and_then(|keyboard| keyboard.inline_keyboard.last())
        .and_then(|last_vec| last_vec.last())
        .and_then(|last_button| match last_button.text.as_str() {
            // If the last button is not CREATE_A_PROPOSAL then it's SEE_ALL_PROPOSALS
            SUBMIT_A_PROPOSAL => Some(SubMenuType::CreateNewProposal),
            _ => Some(SubMenuType::SeeProposals),
        })
        .ok_or_else(|| {
            anyhow::anyhow!("find_sub_menu_type_from_callback: No valid sub menu found")
        });
    res.ok()
}

pub fn find_keyboard_from_message(msg: &Message) -> anyhow::Result<&InlineKeyboardMarkup> {
    msg.reply_markup()
        .ok_or_else(|| anyhow::anyhow!("find_keyboard_from_message: No valid sub menu found"))
}

pub async fn delete_up_to_messages(
    bot: &Bot,
    chat_id: i64,
    start: i32,
    end: i32,
) -> Result<(), TgError> {
    println!("chat_id: {}, start: {}, end: {}", chat_id, start, end);
    for message_id in (end + 1..=start).rev() {
        sleep(Duration::from_millis(10)).await;
        let _ = bot
            .delete_message(ChatId(chat_id), MessageId(message_id))
            .await;
    }
    Ok(())
}
