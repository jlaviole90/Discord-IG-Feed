use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth;
use crate::models::{Embeds, IGAccount, Post, Root};
use crate::proxy::*;
use rand::Rng;
use reqwest::{Client, Proxy};
use serenity::prelude::TypeMapKey;
use std::time::Duration;

pub struct IGChannel {
    proxies: Vec<GeoNodeProxy>,
    prx_iter: usize,
    account: String,
    pub last_post: Post,
    pub last_fetch: SystemTime,
}
impl TypeMapKey for IGChannel {
    type Value = String;
}

impl Default for IGChannel {
    fn default() -> Self {
        IGChannel {
            proxies: vec![
                PROXY0, PROXY1, PROXY2, PROXY3, PROXY4, PROXY5, PROXY6, PROXY7, PROXY8, PROXY9,
                PROXY10,
            ],
            prx_iter: 0,
            account: String::new(),
            last_post: Post::default(),
            last_fetch: UNIX_EPOCH,
        }
    }
}
impl IGChannel {
    pub fn init(account: &str) -> Self {
        let mut this = IGChannel::default();
        this.account = account.to_string();
        this
    }
    fn fetch_proxies(&mut self) -> Proxy {
        // Grab a random GeoNode proxy.
        self.prx_iter = rand::thread_rng().gen_range(0..10);
        let cur_prx: &GeoNodeProxy = self.proxies.get(self.prx_iter).unwrap();
        // Connect to random HTTPS proxy.
        Proxy::https(format!(
            "{current_prx}:{current_prt}",
            current_prx = cur_prx.ip,
            current_prt = cur_prx.port
        ))
        .unwrap()
        .basic_auth(auth::get_prx_user().as_str(), auth::get_prx_pass().as_str())
    }

    pub async fn deploy_proxy_server(&mut self) {
        loop {
            let latest_post = match Client::builder()
                .proxy(self.fetch_proxies())
                .user_agent(USER_AGENT)
                .build()
                .expect("Failed to build HTTP client... ")
                // todo: get the user name!
                .get(IG_HOST.to_owned() + self.account.as_str())
                .send()
                .await
            {
                Ok(resp) => match resp.json::<Root>().await {
                    Ok(data) => data,
                    Err(why) => {
                        println!("Failed to decode IG response: \n{:?},\n", why);
                        return;
                    }
                },
                Err(why) => {
                    println!("Failed to retrieve IG data.\n{:?}\n", why);
                    return;
                }
            }
            .data
            .user
            .edge_owner_to_timeline_media
            .edges
            .iter()
            .map(|ed| ed.node.to_owned())
            .filter(|nd| nd.pinned_for_users.len() <= 0)
            .next()
            .expect("Unable to parse JSON posts.");

            // todo: insert proper username
            let last_post = Post {
                username: "".to_string(),
                embeds: Embeds {
                    description: latest_post
                        .edge_media_to_caption
                        .edges
                        .first()
                        .unwrap()
                        .node
                        .text
                        .to_string(),
                    image: if latest_post.is_video {
                        latest_post.video_url.unwrap()
                    } else {
                        latest_post.display_url
                    },
                    timestamp: latest_post.taken_at_timestamp,
                },
            };

            self.last_post = last_post;
            self.last_fetch = SystemTime::now();
            tokio::time::sleep(Duration::from_secs(120)).await;
        }
    }

    pub async fn search(&mut self, account: &str) -> Result<IGAccount, String> {
        let user = match Client::builder()
            .proxy(self.fetch_proxies())
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build HTTP client... ")
            .get(IG_HOST.to_owned() + account)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Root>().await {
                Ok(data) => data,
                Err(why) => return Err(format!("Failed to decode IG response: \n{:?},\n", why)),
            },
            Err(why) => return Err(format!("Failed to retrieve IG data.\n{:?}\n", why)),
        }
        .data
        .user;

        Ok(IGAccount {
            username: user.username,
            bio: user.biography,
            profile_pic: user.profile_pic_url_hd,
        })
    }
}
