use std::env;

pub fn get_prx_user() -> String {
    env::var("PROXY_USER").expect("Proxy username not found.")
}

pub fn get_prx_pass() -> String {
    env::var("PROXY_PASS").expect("Proxy password not found.")
}

pub const IG_HOST: &str = "https://www.instagram.com/api/v1/users/web_profile_info/?username=";
pub const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 9; GM1903 Build/PKQ1.190110.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/75.0.3770.143 Mobile Safari/537.36 Instagram 103.1.0.15.119 Android (28/9; 420dpi; 1080x2260; OnePlus; GM1903; OnePlus7; qcom; sv_SE; 164094539)";

pub struct GeoNodeProxy {
    pub ip: &'static str,
    pub port: &'static str,
}

pub const PROXY0: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9000",
};
pub const PROXY1: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9001",
};
pub const PROXY2: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9002",
};
pub const PROXY3: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9003",
};
pub const PROXY4: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9004",
};
pub const PROXY5: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9005",
};
pub const PROXY6: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9006",
};
pub const PROXY7: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9007",
};
pub const PROXY8: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9008",
};
pub const PROXY9: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9009",
};
pub const PROXY10: GeoNodeProxy = GeoNodeProxy {
    ip: "51.159.149.67",
    port: "9010",
};
