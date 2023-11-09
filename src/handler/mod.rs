pub mod callback_handlers;

use crate::consts::CREATE_A_PROPOSAL;
use teloxide::types::{CallbackQuery, InlineKeyboardMarkup};

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
            CREATE_A_PROPOSAL => Some(SubMenuType::CreateNewProposal),
            _ => Some(SubMenuType::SeeProposals),
        })
        .ok_or_else(|| {
            anyhow::anyhow!("find_sub_menu_type_from_callback: No valid sub menu found")
        });
    res.ok()
}

pub fn find_keyboard_from_callback(q: &CallbackQuery) -> anyhow::Result<&InlineKeyboardMarkup> {
    q.message
        .as_ref()
        .and_then(|msg| msg.reply_markup())
        .and_then(|keyboard| Some(keyboard))
        .ok_or_else(|| anyhow::anyhow!("find_sub_menu_type_from_callback: No valid sub menu found"))
}
