use reqwest::{Client, Proxy};
use crate::models::{Post, Embeds, Node3, Root};

const IG_HOST: &str = "https://www.instagram.com/api/v1/users/web_profile_info/?username=jamescagewhite";
const PROXY_URL: &str = "https://api.proxyscrape.com/v2/?request=displayproxies&protocol=http&country=us&ssl=all&anonymity=elite";
const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 9; GM1903 Build/PKQ1.190110.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/75.0.3770.143 Mobile Safari/537.36 Instagram 103.1.0.15.119 Android (28/9; 420dpi; 1080x2260; OnePlus; GM1903; OnePlus7; qcom; sv_SE; 164094539)";

pub struct IGChannel {
    proxies: Vec<Proxy>
}
impl Default for IGChannel {
    fn default() -> Self {
        IGChannel {
            proxies: Vec::new()
        }
    }
}

impl IGChannel {
    async fn fetch_proxies() -> Vec<Proxy> {
        // Collect a set of proxies and map them to the reqwest struct.
        let client = Client::new();
        client.get(PROXY_URL).send()
            .await.expect("Unable to get proxies from provider.")
            .text()
            .await.expect("Failed to deserialize proxy text")
            .lines()
            .map(|str| -> Proxy {
                Proxy::http(str.to_string()).unwrap()
            })
            .collect()
    }
    pub async fn rec_new(&mut self) -> Post {
        // Ensure there are still proxies in the list, and pop the next one.
        let mut proxy: Option<Proxy> = self.proxies.pop();
        match proxy {
            None => {
                // Refill the proxy list and set the next one.
                self.proxies = IGChannel::fetch_proxies().await;
                proxy = self.proxies.pop();
            }
            _ => {}
        };

        // Build the client with necessary headers, and the next proxy
        let client: Client = Client::builder()
            .proxy(proxy.expect("Failed to pull a fresh proxy."))
            .user_agent(USER_AGENT)
            .build()
            .unwrap();

        let json_resp = client.get(IG_HOST).send().await;
        if let Err(why) = json_resp {
            panic!("Failed to retrieve IG data: \n{:?}\n", why)
        }

        let root = json_resp.unwrap().json::<Root>().await;
        if let Err(why) = root {
            panic!("Failed to decode IG response: \n{:?},\n", why)
        }

        let latest_post: Node3 = root.unwrap()
            .data.user.edge_owner_to_timeline_media.edges
            .iter()
            .map(
                |ed| ed.node.to_owned()
            ).filter(
                |nd| nd.pinned_for_users.len() <= 0
            ).next().unwrap();

        Post {
            username: "jamescagewhite".to_string(),
            embeds: vec![
                Embeds {
                    description: latest_post.edge_media_to_caption.edges.first().unwrap().node.text.to_string(),
                    image: if latest_post.is_video { latest_post.video_url.unwrap() } else { latest_post.display_url },
                    timestamp: latest_post.taken_at_timestamp
                }
            ]
        }
    }
}
