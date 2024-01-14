use std::env;

pub struct Config {
    pub port: u16,
    pub host: String,
    pub source: String,
    pub implements: Vec<String>
}

fn get_env<T>(key: &'static str, default: T) -> T
where
    T: std::str::FromStr,
{
    match env::var(key) {
        Ok(value) => match value.parse::<T>() {
            Ok(parsed_value) => parsed_value,
            Err(_) => default,
        },
        Err(_) => default,
    }
}

pub fn get_config() -> Config {
    Config {
        port: get_env("PORT", 8080),
        host: get_env("HOST", "0.0.0.0".to_string()),
        source: get_env("SOURCE", "/".to_string()),
        implements: get_implements(),
    }
}

fn get_implements() -> Vec<String> {
    let mut implements: Vec<String> = vec![];
    for implement in get_env("IMPLEMENTS", "".to_string()).split(',') {
        implements.push(implement.to_string());
    }
    implements
}