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

#[tokio::main]
async fn main() {
    let reply_use_case = Arc::new(ReplyUseCase::new());

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
