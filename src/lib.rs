use std::fmt::{Debug, Display, Formatter};

use serenity::async_trait;
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::framework::standard::macros::{command, group};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::bot::commands_discord::generate_regex_command::generate_regex_command;

mod core;
mod bot;
mod models;

#[group]
#[commands(reg_generate)]
struct General;

#[command]
async fn reg_generate(ctx: &Context, msg: &Message) -> CommandResult {
    generate_regex_command(ctx, msg).await
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

struct Counter;
impl TypeMapKey for Counter {
    type Value = u32;
}

#[derive(Debug)]
struct MonErreur;

impl Display for MonErreur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mon Erreur")
    }
}

impl std::error::Error for MonErreur {}

pub async fn run_discord_bot(token: &str) {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
