use crate::consts::{CREATE_A_PROPOSAL, MAIN_MENU, SEE_PROPOSALS};
use crate::handler::callback_handlers::{
    handle_menu_callback, handle_new_proposal_callback, handle_proposal_fields_callback,
    handle_see_proposals_callback, handle_submit_proposal_callback, handle_thumb_up_callback,
};
use crate::handler::dialogue_handlers::{
    receive_description_handler, receive_expiration_date_handler, receive_starting_date_handler,
    receive_title_handler, start_title_dialogue_handler, DialogueState,
};
use crate::handler::{match_sub_menu, SubMenuType};
use crate::keyboards::see_proposals_keyboard::SeeProposalsKeyboard;
use crate::storage::{TgMessage, TgMessageStorage, GLOBAL_MAIN_MENU_STORAGE};
use crate::utils::delete_previous_messages;
use crate::TgError;
use crate::{
    keyboards::{create_new_proposal_keyboard::CreateNewProposalKeyboard, menu_keyboard},
    messages::get_welcome_message,
};
use core::time::Duration;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use teloxide::{
    dispatching::{dialogue::InMemStorage, HandlerExt, UpdateFilterExt},
    dptree,
    error_handlers::LoggingErrorHandler,
    payloads::SendMessageSetters,
    prelude::{Dispatcher, Requester},
    types::{CallbackQuery, Message, ParseMode, Update},
    utils::command::BotCommands,
    Bot,
};
use tokio::time::sleep;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Show Available Commands")]
    Help,
    #[command(description = "Main Menu")]
    Menu,
    #[command(description = "Start the bot")]
    Start,
}

#[derive(Clone, Debug)]
pub struct TgBot {
    bot: Bot,
}

impl TgBot {
    pub fn new() -> Self {
        dotenv().ok();
        let api_key = env::var("TG_BOT_API_KEY").expect("TG_BOT_API_KEY not set");
        let bot = Bot::new(api_key);
        Self { bot }
    }

    pub async fn init(self) -> Result<(), TgError> {
        let handler = dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .endpoint(command_callback),
            )
            .branch(Update::filter_callback_query().endpoint(button_callback))
            .branch(
                Update::filter_message()
                    .enter_dialogue::<Message, InMemStorage<DialogueState>, DialogueState>()
                    .branch(
                        dptree::case![DialogueState::StartTitlePrompt]
                            .endpoint(start_title_dialogue_handler),
                    )
                    .branch(
                        dptree::case![DialogueState::TitleReceived].endpoint(receive_title_handler),
                    )
                    .branch(
                        dptree::case![DialogueState::DescriptionReceived]
                            .endpoint(receive_description_handler),
                    )
                    .branch(
                        dptree::case![DialogueState::StartingDateReceived]
                            .endpoint(receive_starting_date_handler),
                    )
                    .branch(
                        dptree::case![DialogueState::ExpirationDateReceived]
                            .endpoint(receive_expiration_date_handler),
                    ),
            );

        Dispatcher::builder(self.bot, handler)
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .dependencies(dptree::deps![InMemStorage::<DialogueState>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        Ok(())
    }
}

async fn command_callback(bot: Bot, cmd: Command, msg: Message) -> Result<(), TgError> {
    match cmd {
        Command::Help => {
            let _ = bot
                .send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Menu => {
            let keyboard = menu_keyboard();
            let welcome_msg = get_welcome_message();

            // send the new message
            let message_sent = bot
                .send_message(msg.chat.id, welcome_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;
            let message_sent = Arc::new(message_sent);

            // Updates the GLOBAL_MAIN_MENU_STORAGE
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

            // delete previous messages
            let last_message_id = message_sent.id;
            let _ =
                delete_previous_messages(&bot, msg.chat.id.0, last_message_id.0 - 1, 20).await?;
        }
        Command::Start => {
            sleep(Duration::from_secs(1)).await;
            let keyboard = menu_keyboard();
            let menu_msg = get_welcome_message();

            // send the welcome message
            let _message_sent = bot
                .send_message(msg.chat.id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;
        }
    }
    Ok(())
}

async fn button_callback(
    bot: Bot,
    q: CallbackQuery,
    storage: Arc<InMemStorage<DialogueState>>,
) -> Result<(), TgError> {
    if let Some(action) = &q.data {
        match action.as_str() {
            CREATE_A_PROPOSAL => handle_new_proposal_callback(&bot, &q).await?,
            SEE_PROPOSALS => handle_see_proposals_callback(&bot, &q).await?,
            MAIN_MENU => handle_menu_callback(&bot, &q).await?,
            _ => match match_sub_menu(&q) {
                Some(SubMenuType::CreateNewProposal) => {
                    match CreateNewProposalKeyboard::new(action) {
                        CreateNewProposalKeyboard::Title(_) => {
                            handle_proposal_fields_callback(
                                &bot,
                                DialogueState::TitleReceived,
                                &q,
                                storage,
                                CreateNewProposalKeyboard::Title(""),
                            )
                            .await?
                        }
                        CreateNewProposalKeyboard::Description(_) => {
                            handle_proposal_fields_callback(
                                &bot,
                                DialogueState::DescriptionReceived,
                                &q,
                                storage,
                                CreateNewProposalKeyboard::Description(""),
                            )
                            .await?
                        }
                        CreateNewProposalKeyboard::StartingDate(_) => {
                            handle_proposal_fields_callback(
                                &bot,
                                DialogueState::StartingDateReceived,
                                &q,
                                storage,
                                CreateNewProposalKeyboard::StartingDate(""),
                            )
                            .await?
                        }
                        CreateNewProposalKeyboard::ExpirationDate(_) => {
                            handle_proposal_fields_callback(
                                &bot,
                                DialogueState::ExpirationDateReceived,
                                &q,
                                storage,
                                CreateNewProposalKeyboard::ExpirationDate(""),
                            )
                            .await?
                        }
                        _ => handle_submit_proposal_callback(&bot, &q).await?,
                    }
                }
                Some(SubMenuType::SeeProposals) => match SeeProposalsKeyboard::new(action) {
                    SeeProposalsKeyboard::ThumbUp => {
                        handle_thumb_up_callback(&bot, &q, SeeProposalsKeyboard::ThumbUp).await?
                    }
                },
                _ => {}
            },
        }
    }
    Ok(())
}
