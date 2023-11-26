use std::ops::Deref;
use std::path::Path;
use reqwest::*;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serde_json::Value;
use image::*;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::CommandError;
use crate::commands::{FEED_INIT, FEED_START, TEST, TEST_RESP};

#[derive(Serialize, Deserialize)]
struct Post {
    username: String,
    avatar_url: String,
    shortCode: String,
    embeds: Vec<Embeds>
}
impl Default for Post {
    fn default () -> Post {
        Post {
            username: "".parse().unwrap(),
            avatar_url: "".parse().unwrap(),
            shortCode: "".parse().unwrap(),
            embeds: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Embeds {
    title: String,
    color: String,
    url: String,
    description: String,
    author: Author,
    image: String,
    footer: Footer
}

#[derive(Serialize, Deserialize)]
struct Author {
    name: String,
    icon_url: String
}

#[derive(Serialize, Deserialize)]
struct Footer {
    icon_url: String,
    text: String
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message is received - the
    // closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a thread pool, and so multiple events can be dispatched
    // simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == TEST {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, TEST_RESP).await {
                println!("Error sending message: {why:?}");
            }
        }
        // Initialize the channel where the bot will post updates
        if msg.content == FEED_START {
            let channel: ChannelId = msg.channel_id;

            // if let Err(why) = channel.say(&ctx.http, FEED_INIT).await {
            //     println!("Error sending message: {why:?}");
            // }
            // prompt user to begin now or fetch all posts?
            // Feed::new(msg.channel_id, ctx).listen();

            let response = get("https://jcw-26b2f638ef03.herokuapp.com/get-latest")
                .await
                .expect("Failed to retrieve newest post")
                .json::<Post>()
                .await;

            let mut post: Post = Post::default();
            if let Err(why) = response {
                println!("JSON not valid: {why:?}")
            } else {
                post = response.unwrap();
            }

            match msg.channel_id.send_message(&ctx.http, |m| {
                m.add_file(AttachmentType::Image(
                    Url::parse(&post.embeds.first().unwrap().image).unwrap()));
                m.content(&post.embeds.first().unwrap().description);
                m
            }).await {
                Ok(_) => Ok(()),
                Err(why) => Err(CommandError::from(why)),
            }.expect("Posting to discord failed");
        }

    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}