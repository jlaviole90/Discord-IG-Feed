use chrono::prelude::DateTime;
use reqwest::*;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::CommandError;
use serenity::http::Http;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::commands::{JAMES, NEW, POSTS, TEST, TEST_RESP};
use crate::models::{Embeds, Post};
use crate::proxy::IGChannel;

const DATE_FORMAT: &str = "%m-%d-%Y %H:%M";

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // Message Event Handler
    async fn message(&self, ctx: Context, msg: Message) {
        let mut ig_channel = IGChannel::default();
        ig_channel.deploy_proxy_server().await;
        /*
         *  Test command to verify the bot is running
         *  "very cool very swag" -> "I like it!"
         */
        if msg.content == TEST {
            if let Err(why) = msg.channel_id.say(&ctx.http, TEST_RESP).await {
                println!("Error sending message: {why:?}");
            }
        }

        /*
         *  grab the latest JCW IG post and create it in discord on command
         *  james new -> <latest IG post>
         */
        if msg.content == format!("{prefix}{command}", prefix = JAMES, command = NEW) {
            let post: &Post = &ig_channel.last_post;
            let emb: &Embeds = &post.embeds;
            post_msg(&ctx.http, emb, &msg).await;
        }

        /*
         *  update the chosen channel indefinitely, with new posts (if available)
         *  being delivered every 2 minutes.
         *
         *  Running two infinite loops in parralel here is slower, but more responsive.
         */
        if msg.content == format!("{prefix}{command}", prefix = JAMES, command = POSTS) {
            let mut last_stmp: SystemTime = SystemTime::now();
            loop {
                println!("Good morning!");
                if ig_channel.last_fetch != last_stmp {
                    println!("Very cool very swag I like it!");
                    last_stmp = ig_channel.last_fetch;
                    post_msg(&ctx.http, &ig_channel.last_post.embeds, &msg).await;
                }
                println!("zzzZZZzzzZZZzzzZZZ\n");
                tokio::time::sleep(Duration::from_secs(120)).await;
            }
        }
    }

    // Handler used for the "ready" event.
    // Print what the Bot username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected\n", ready.user.name);
    }
}

async fn post_msg(http: &Arc<Http>, emb: &Embeds, msg: &Message) {
    match msg
        .channel_id
        .send_message(http, |m| {
            m.add_file(AttachmentType::Image(Url::parse(&emb.image).unwrap()));
            m.content(format!(
                "{time} \n {desc}",
                time = DateTime::from_timestamp(emb.timestamp, 0)
                    .unwrap()
                    .format(DATE_FORMAT)
                    .to_string(),
                desc = emb.description
            ));
            return m;
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(why) => Err(CommandError::from(why)),
    }
    .expect("Posting to discord failed");
}
