use std::sync::Arc;
use tokio::sync::Mutex;

use teloxide::{prelude::Requester, types::Message, Bot};
use tracing::info;

use crate::{
    domain::{self, use_case::ReplyUseCase},
    infrastructure::repositories::repository::Repository,
};

pub async fn handle_message<R: Repository>(
    bot: Bot,
    message: Message,
    use_case: Arc<Mutex<ReplyUseCase<R>>>,
) -> Result<(), teloxide::errors::RequestError> {
    let chat_id = message.chat.id.to_string();
    let from = message.from.as_ref().unwrap();
    let username = from.username.as_ref().unwrap();
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
            let use_case_message =
                domain::models::Message::new(username, text, &chat_id, time, is_bot_mentioned);

            let mut use_case = use_case.lock().await;

            let response = use_case.execute(use_case_message).await;
            if response.is_some() {
                bot.send_message(chat_id, response.unwrap()).await?;
            }
        }
        None => return Ok(()),
    }

    Ok(())
}
