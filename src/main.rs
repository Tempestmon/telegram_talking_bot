mod domain;
mod handlers;
mod infrastructure;

use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    prelude::Dispatcher,
    types::{Message, Update},
    Bot,
};

use domain::use_case::ReplyUseCase;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::infrastructure::repositories::basic_repository::BasicRepository;

#[tokio::main]
async fn main() {
    let basic_repository = BasicRepository::new();
    let reply_use_case = Arc::new(Mutex::new(ReplyUseCase::new(basic_repository)));

    let bot = Bot::from_env();
    let handler = dptree::entry().branch(Update::filter_message().endpoint(
        move |bot: Bot, message: Message| {
            handlers::telegram::handle_message(bot, message, Arc::clone(&reply_use_case))
        },
    ));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
