mod ig_feeder;
mod events;
mod commands;

use std::collections::HashSet;

use serenity::framework::standard::{CommandGroup, help_commands, CommandResult, HelpOptions, Args, StandardFramework};
use serenity::framework::standard::macros::help;
use serenity::http::Http;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::channel::Message;
use crate::commands::{FATE, JAMES};

const DISCORD_TOKEN: &str = "";

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {

    // TODO: setup real help display
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = DISCORD_TOKEN;
    let http = Http::new(&token);

    // Get the bot ID to allow mentions
    let bot_id = match http.get_current_user().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access user info {:?}", why),
    };

    // Setup framework to allow mentions, and use the "james " prefix
    let framework = StandardFramework::new()
        .help(&MY_HELP)
        .configure(|c| c
            .on_mention(Some(bot_id))
            .prefix(JAMES));

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;


    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(events::Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
