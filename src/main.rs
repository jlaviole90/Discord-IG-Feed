use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::gateway::Ready;
use serenity::model::Timestamp;

const DISCORD_TOKEN: &str = "MTE3Nzg1MTIyODA2NDA3MTc4MA.GBU9DP.TWWOtQS70Az9iUUzSLo42u-9tsNikTjsbYfRPM";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = DISCORD_TOKEN;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

// #[command]
// async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.reply(ctx, "Pong!").await?;
//     Ok::<(), ()>(())
// }
