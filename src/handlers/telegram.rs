use std::sync::Arc;

use teloxide::{prelude::Requester, types::Message, Bot};
use tracing::{info, warn};

use crate::{
    domain::{self, use_case::ReplyUseCase},
    infrastructure::repositories::repository::Repository,
};

pub async fn handle_message<R: Repository>(
    bot: Bot,
    message: Message,
    use_case: Arc<ReplyUseCase<R>>,
) -> Result<(), teloxide::errors::RequestError> {
    info!("Got message to handle");
    let chat_id = message.chat.id.to_string();
    let is_private = message.chat.is_private();
    let from = message.from.as_ref().unwrap();
    let username = from
        .username
        .clone()
        .unwrap_or_else(|| "Unknown".to_string());
    let mut is_bot_mentioned = false;
    let bot_name = &bot.get_me().await.unwrap().username.clone().unwrap();
    let text = message.text();
    let time = message.date;
    match text {
        Some(text) => {
            info!("Receive message {text} from username {username} in chat {chat_id}");
            if text.contains(bot_name) {
                is_bot_mentioned = true;
            }
            let use_case_message = domain::models::Message::new(
                username.as_str(),
                text,
                &chat_id,
                time,
                is_bot_mentioned,
                is_private,
            );

            if let Some(response) = use_case.execute(use_case_message).await {
                info!("Got response from AI: {response}");
                bot.send_message(chat_id, response).await?;
            }
        }
        None => warn!("Message does not contains text"),
    }

    Ok(())
}
