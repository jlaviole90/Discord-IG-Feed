// use std::thread;
// use std::time::Duration;
// use instagram_scraper_rs::Post;
// use serenity::client::Context;
// use serenity::model::id::ChannelId;
//
// pub struct Feed {
//     channel_id: ChannelId,
//     context: Context,
// }
//
// impl Feed {
//     pub fn new(c_id: ChannelId, ctx: Context) -> Self {
//         Self {
//             channel_id: c_id,
//             context: ctx
//         }
//     }
//     pub fn listen(&mut self) {
//         loop {
//             // wait 5 minutes between updates
//             thread::sleep(Duration::from_secs(600));
//
//             // get new posts
//
//             // push new posts to the queue
//             // if post exists {
//             self.post(Post::new)
//         };
//     }
//
//     async fn post(&mut self, post: Post) {
//         self.channel_id.say(&self.context.http, post).await.expect("Error posting update");
//     }
// }
//
//
//
// // using builder for future expansion
// pub struct FeedBuilder {
//     channel_id: ChannelId,
//     context: Context
// }
//
// impl FeedBuilder {
//     fn _new(channel: ChannelId, ctx: Context) -> Self {
//         Self {
//             channel_id: channel,
//             context: ctx
//         }
//     }
// }
