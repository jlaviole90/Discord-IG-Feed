// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;

use crate::commands::JAMES;
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
mod models;
mod proxy;

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
async fn search_account(account: &str) -> Result<String, String> {
    Ok(account.to_string())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// todo: possible to return a string streamof console output???
#[tauri::command]
async fn start_server(token: &str) -> Result<bool, String> {
    let token = token;
    let http = Http::new(&token);

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
        .configure(|c| c.on_mention(Some(bot_id)).prefix(JAMES));

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
