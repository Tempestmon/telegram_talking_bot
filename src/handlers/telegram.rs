use std::sync::Arc;

use teloxide::{prelude::Requester, types::Message, Bot};

use crate::domain::use_case::ReplyUseCase;

pub async fn handle_message(
    bot: Bot,
    message: Message,
    use_case: Arc<ReplyUseCase>,
) -> Result<(), teloxide::errors::RequestError> {
    let chat_id = message.chat.id;
    let from = message.from.as_ref().unwrap();
    let username = from.username.as_ref().unwrap();
    let text = message.text().expect("Message is not a text");
    let message_from_user = format!("{username}: {text}");

    let response = use_case.execute(message_from_user).await;

    bot.send_message(chat_id, response).await?;

    Ok(())
}
