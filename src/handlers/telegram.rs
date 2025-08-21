use std::sync::Arc;
use tokio::sync::Mutex;

use teloxide::{prelude::Requester, types::Message, Bot};

use crate::{domain::use_case::ReplyUseCase, infrastructure::repositories::repository::Repository};

pub async fn handle_message<R: Repository>(
    bot: Bot,
    message: Message,
    use_case: Arc<Mutex<ReplyUseCase<R>>>,
) -> Result<(), teloxide::errors::RequestError> {
    let chat_id = message.chat.id;
    let from = message.from.as_ref().unwrap();
    let username = from.username.as_ref().unwrap();
    let text = message.text().expect("Message is not a text");
    let message_from_user = format!("{username}: {text}");

    let mut use_case = use_case.lock().await;

    let response = use_case.execute(message_from_user).await;

    bot.send_message(chat_id, response).await?;

    Ok(())
}
