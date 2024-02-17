use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth;
use crate::models::{Embeds, Post, Root};
use rand::Rng;
use reqwest::{Client, Proxy};
use std::time::Duration;

const IG_HOST: &str =
    "https://www.instagram.com/api/v1/users/web_profile_info/?username=jamescagewhite";
const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 9; GM1903 Build/PKQ1.190110.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/75.0.3770.143 Mobile Safari/537.36 Instagram 103.1.0.15.119 Android (28/9; 420dpi; 1080x2260; OnePlus; GM1903; OnePlus7; qcom; sv_SE; 164094539)";
const PROXY0:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9000" };
const PROXY1:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9001" };
const PROXY2:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9002" };
const PROXY3:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9003" };
const PROXY4:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9004" };
const PROXY5:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9005" };
const PROXY6:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9006" };
const PROXY7:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9007" };
const PROXY8:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9008" };
const PROXY9:  GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9009" };
const PROXY10: GeoNodeProxy = GeoNodeProxy { ip: "51.159.149.67", port: "9010" };

struct GeoNodeProxy {
    ip: &'static str,
    port: &'static str,
}
pub struct IGChannel {
    proxies: Vec<GeoNodeProxy>,
    prx_iter: usize,
    pub last_post: Post,
    pub last_fetch: SystemTime,
}
impl Default for IGChannel {
    fn default() -> Self {
        IGChannel {
            proxies: vec![PROXY0, PROXY1, PROXY2, PROXY3, PROXY4, PROXY5, PROXY6, PROXY7, PROXY8, PROXY9, PROXY10],
            prx_iter: 0,
            last_post: Post::default(),
            last_fetch: UNIX_EPOCH,
        }
    }
}

impl IGChannel {
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
                .get(IG_HOST)
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

            let last_post = Post {
                username: "jamescagewhite".to_string(),
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
}
