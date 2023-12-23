use std::string::ToString;
use rand::{Rng};
use reqwest::{Client, Proxy};
use crate::models::{Post, Embeds, Node3, Root};

const IG_HOST: &str = "https://www.instagram.com/api/v1/users/web_profile_info/?username=jamescagewhite";
const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 9; GM1903 Build/PKQ1.190110.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/75.0.3770.143 Mobile Safari/537.36 Instagram 103.1.0.15.119 Android (28/9; 420dpi; 1080x2260; OnePlus; GM1903; OnePlus7; qcom; sv_SE; 164094539)";
struct GeoNodeProxy {
    ip: String,
    port: String
}
pub struct IGChannel {
    proxies: Vec<GeoNodeProxy>,
    prx_iter: usize
}
impl Default for IGChannel {
    fn default() -> Self {
        IGChannel {
            proxies: vec![
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9000".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9001".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9002".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9003".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9004".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9005".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9006".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9007".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9008".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9009".to_string()},
                GeoNodeProxy {ip: "51.159.149.67".to_string(), port: "9010".to_string()}],
            prx_iter: 0
        }
    }
}

impl IGChannel {
    fn fetch_proxies(&mut self) -> Proxy {
        // Grab a random GeoNode proxy.
        self.prx_iter = rand::thread_rng().gen_range(0..10);
        let cur_prx: &GeoNodeProxy = self.proxies.get(self.prx_iter).unwrap();
        // Connect to random HTTPS proxy.
        Proxy::https(format!("{current_prx}:{current_prt}",
            current_prx=cur_prx.ip,
            current_prt=cur_prx.port)
        ).unwrap()
            .basic_auth(
                "",
                "")
    }
    pub async fn rec_new(&mut self) -> Post {
        // Build the client with necessary headers, and the next proxy
        let proxy = self.fetch_proxies();
        let root =
            match Client::builder()
                .proxy(proxy)
                .user_agent(USER_AGENT)
                .build().expect("Failed to build HTTP client...")
                .get(IG_HOST).send().await {
                Ok(resp) => {
                    match resp.json::<Root>().await {
                        Ok(data) => data,
                        Err(why) => {
                            println!("Failed to decode IG response: \n{:?},\n", why);
                            return Post::default();
                        }
                    }
                },
                Err(why) => {
                    println!("Failed to retrieve IG data.\n{:?}\n", why);
                    return Post::default();
                }
            };

        let latest_post: Node3 = root
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
