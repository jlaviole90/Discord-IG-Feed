use chrono::prelude::DateTime;
use reqwest::*;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::CommandError;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;

use crate::commands::{JAMES, NEW, TEST, TEST_RESP};
use crate::models::{Embeds, Post};

const API: &str = "https://jcw-26b2f638ef03.herokuapp.com/get-latest";
const DATE_FORMAT: &str = "%m-%d-%Y %H:%M";

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // Message Event Handler
    async fn message(&self, ctx: Context, msg: Message) {
        /*
         *  Test command to verify the bot is running
         *  "very cool very swag" -> "I like it!"
         */
        if msg.content == TEST {
            if let Err(why) = msg.channel_id.say(&ctx.http, TEST_RESP).await {
                println!("Error sending message: {why:?}");
            }
        }
        // TODO: Initialize the channel where the bot will post all new updates
        // let channel: ChannelId = msg.channel_id;
        // Feed::new(msg.channel_id, ctx).listen();

        /*
         *  grab the latest JCW IG post and create it in discord on command
         *  james new -> <latest IG post>
         */
        if msg.content == format!("{prefix}{command}", prefix = JAMES, command = NEW) {
            let mut post: Post = Post::default();
            let response = get(API)
                .await
                .expect("Failed to retrieve newest post")
                .json::<Post>()
                .await;
            if let Err(why) = response {
                println!("JSON not valid: {why:?}")
            } else {
                post = response.unwrap();
            }

            let emb: &Embeds = post.embeds.first().unwrap();

            match msg.channel_id.send_message(&ctx.http, |m| {
                m.add_file(AttachmentType::Image(
                    Url::parse(&emb.image)
                        .unwrap())
                );
                m.content(
                    format!("{time} \n {desc}",
                            time = DateTime::from_timestamp(emb.timestamp, 0)
                                .unwrap()
                                .format(DATE_FORMAT)
                                .to_string(),
                            desc = emb.description)
                );
                return m;
            }).await {
                Ok(_) =>
                    Ok(()),
                Err(why) =>
                    Err(CommandError::from(why)),
            }.expect("Posting to discord failed");
        }
    }

    // Handler used for the "ready" event.
    // Print what the Bot username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}