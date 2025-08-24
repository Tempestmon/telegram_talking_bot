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
use tracing::info;

use domain::use_case::ReplyUseCase;
use std::sync::Arc;

use crate::infrastructure::{
    adapters::deepseek::DeepSeekAdapter, repositories::basic_repository::BasicRepository,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    info!("Starting Telegram bot");
    let deepseek_adapter = Arc::new(DeepSeekAdapter::new());
    let basic_repository = Arc::new(BasicRepository::new());
    let reply_use_case = Arc::new(ReplyUseCase::new(deepseek_adapter, basic_repository));

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

    info!("Bot stopped");
}
