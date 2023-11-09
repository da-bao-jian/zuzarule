use crate::TgError;
use core::time::Duration;
use teloxide::prelude::Requester;
use teloxide::types::{ChatId, MessageId};
use teloxide::Bot;
use tokio::time::sleep;

/// Helper function to delete number_of_deletes previous messages
pub async fn delete_previous_messages(
    bot: &Bot,
    chat_id: i64,
    last_message_id: i32,
    number_of_deletes: i32,
) -> Result<(), TgError> {
    log::info!("last message id: {}", last_message_id);
    for message_id in (last_message_id - number_of_deletes..=last_message_id).rev() {
        sleep(Duration::from_millis(10)).await;
        let _ = bot
            .delete_message(ChatId(chat_id), MessageId(message_id))
            .await;
    }
    Ok(())
}
