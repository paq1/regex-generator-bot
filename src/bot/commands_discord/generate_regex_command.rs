use regex_generator::core::services::regex_generator::RegexGenerator;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::bot::services::message_service::reply_standard;

pub async fn generate_regex_command(ctx: &Context, msg: &Message) -> CommandResult {
    let result_of_generate = msg.content
        .split_once(" ")
        .map(|(_, content)| {
            let regex = RegexGenerator::full_regex(content, None);
            Ok(regex.to_string())
        })
        .unwrap_or(Err("message vide"));

    let message = result_of_generate
        .unwrap_or_else(|err| err.to_string());

    reply_standard(message.as_str(), ctx, msg)
        .await?;

    Ok(())
}
