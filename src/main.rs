use std::env;

use dotenv::dotenv;

use regex_generator_bot::run_discord_bot;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("token");
    run_discord_bot(token.as_str()).await;
}
