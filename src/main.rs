#[cfg(feature = "dad-jokes")]
mod dad_jokes;

use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot { return; }

        match msg.content.as_str() {
            #[cfg(feature = "dad-jokes")]
            m if m.contains("dad joke") => {
                _ = msg.channel_id.say(&ctx.http, dad_jokes::fetch().await).await;
            },
            #[cfg(feature = "yes-that")]
            m if m.starts_with('^') => {
                _ = msg.channel_id.say(&ctx.http, format!("^ {}", fastrand::choice(RANDOM_ABOVE_MSG).unwrap())).await;
            },
            _ => {},
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").unwrap();
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.unwrap();

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[cfg(feature = "yes-that")]
const RANDOM_ABOVE_MSG: &[&str] = &[
    "yeah",
    "that",
    "trueeee",
    "yes",
];
