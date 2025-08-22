use std::sync::Arc;
use tokio::sync::Mutex;

use teloxide::{prelude::Requester, types::Message, Bot};

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
    let text = message.text();
    let time = message.date;
    match text {
        Some(text) => {
            let use_case_message =
                domain::models::Message::new(username, &text.to_string(), &chat_id, time);

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
