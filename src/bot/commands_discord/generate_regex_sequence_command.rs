use regex_generator::core::services::regex_generator::RegexGenerator;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::bot::services::message_service::reply_standard;

pub async fn generate_regex_sequence_command(ctx: &Context, msg: &Message) -> CommandResult {

    let command_splited = msg.content
        .split(" ")
        .collect::<Vec<_>>();

    let sequence = command_splited
        .clone()
        .into_iter()
        .nth(1)
        .map(|value| value.parse::<u32>().unwrap_or(0))
        .unwrap_or(0);


    if sequence == 0 {
        reply_standard("la commande n'est pas valide. !reg_generate_seq <taille> <chaine>", ctx, msg)
            .await?;
    } else {
        let new_command = command_splited
            .into_iter()
            .enumerate()
            .filter(|(index, _)| index != &1usize)
            .map(|(_, v)| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let result_of_generate = new_command
            .split_once(" ")
            .map(|(_, content)| {
                let regex = RegexGenerator::full_regex(content, Some(sequence));
                Ok(regex.to_string())
            })
            .unwrap_or(Err("message vide"));

        let message = result_of_generate
            .unwrap_or_else(|err| err.to_string());

        reply_standard(message.as_str(), ctx, msg)
            .await?;
    }

    Ok(())
}
