use std::env;

pub fn get_prx_user() -> String {
    env::var("PROXY_USER").expect("Proxy username not found.")
}

pub fn get_prx_pass() -> String {
    env::var("PROXY_PASS").expect("Proxy password not found.")
}
