use std::env;

pub fn get_env(key: &str, default_value: &str) -> String {
    return env::var(key).unwrap_or_else(|_| default_value.to_string());
}
