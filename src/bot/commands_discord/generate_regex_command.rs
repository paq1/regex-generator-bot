use regex_generator::core::services::regex_generator::RegexGenerator;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::bot::services::message_service::reply_standard;

pub async fn generate_regex_command(ctx: &Context, msg: &Message) -> CommandResult {

    let words = msg.content
        .split(" ")
        .into_iter()
        // .map(|elem| elem.to_string())
        .filter(|elem| elem.trim() != "")
        .map(|e| e.to_string())
        .collect::<Vec<_>>();

    let regex = RegexGenerator::full_regex(words.get(1).unwrap_or(&" ".to_string()).as_str(), None);

    reply_standard(regex.as_str(), ctx, msg).await?;
    Ok(())
}
