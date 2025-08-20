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

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    let handler = dptree::entry().branch(Update::filter_message().endpoint(
        move |bot: Bot, message: Message| handlers::telegram::handle_message(bot, message),
    ));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
