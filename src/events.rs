use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use crate::commands::{FEED_INIT, FEED_START, TEST, TEST_RESP};
use crate::ig_feeder::Feed;

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
            if let Err(why) = channel.say(&ctx.http, FEED_INIT).await {
                println!("Error sending message: {why:?}");
            }
            // prompt user to begin now or fetch all posts?
            Feed::new(msg.channel_id, ctx).listen();
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