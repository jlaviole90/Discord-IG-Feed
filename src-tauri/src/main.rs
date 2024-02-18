// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;

use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

mod auth;
mod commands;
mod events;
mod igapi;
mod models;
mod proxy;

const INTENTS_A: GatewayIntents = GatewayIntents::GUILD_MESSAGES;
const INTENTS_B: GatewayIntents = GatewayIntents::DIRECT_MESSAGES;
const INTENTS_C: GatewayIntents = GatewayIntents::MESSAGE_CONTENT;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_account,
            start_server,
            stop_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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

#[tauri::command]
async fn search_account(account: &str) -> Result<models::IGAccount, String> {
    igapi::IGChannel::default().search(account).await
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// todo: possible to return a string streamof console output???
#[tauri::command]
async fn start_server(token: &str, account: &str, prefix: &str) -> Result<bool, String> {
    let http = Http::new(token);

    let bot_id = match http.get_current_user().await {
        Ok(info) => info.id,
        Err(why) => {
            return Err(format!(
                "Could not access user info {why:?}, bad token input!\nCheck token expiration!"
            ))
        }
    };

    let framework = StandardFramework::new()
        .help(&MY_HELP)
        .configure(|c| c.on_mention(Some(bot_id)).prefix(prefix));

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, INTENTS_A | INTENTS_B | INTENTS_C)
        .event_handler(events::Handler)
        .framework(framework)
        .type_map_insert::<igapi::IGChannel>(account.to_string())
        .type_map_insert::<events::Handler>(prefix.to_string())
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
        Err("{why:?}".to_string())
    } else {
        Ok(true)
    }
}

#[tauri::command]
async fn stop_server() -> Result<bool, String> {
    // todo: likely will have to implement this into a server struct
    Ok(true)
}
