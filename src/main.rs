use std::collections::HashSet;

use serenity::framework::standard::{Args, CommandGroup, CommandResult, help_commands, HelpOptions, StandardFramework};
use serenity::framework::standard::macros::help;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::JAMES;

mod ig_feeder;
mod events;
mod commands;
mod models;



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
    let token = DISCORD_TOKEN;
    let http = Http::new(&token);

    let bot_id = match http.get_current_user().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access user info {:?}", why),
    };

    let framework = StandardFramework::new()
        .help(&MY_HELP)
        .configure(|c| c
            .on_mention(Some(bot_id))
            .prefix(JAMES));

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
