use log::debug;
use reqwest;
use regex::Regex;
use serde::Deserialize;
use serde_json::{self, Value, Map};

#[derive(Deserialize)]
struct ImplResponse {
    // status: String,
    // retcode: u16,
    data: Option<Map<String, Value>>
}

async fn get_nickname(user_id: &String, implement: &String) -> Result<String, ()> {
    let url = format!("{}/get_stranger_info?user_id={}", implement, user_id);
    debug!("Getting {}", url);
    let text = match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(t) => t,
            Err(_) => return Err(())
        },
        Err(_) => return Err(())
    };
    let data: ImplResponse = match serde_json::from_str(&text) {
        Ok(ret) => ret,
        Err(_) => return Err(())
    };
    get_nickname_from_response(data)
}

fn get_nickname_from_response(response: ImplResponse) -> Result<String, ()> {
    match response.data {
        Some(data) => match data.get("nickname") {
            Some(nickname) => match nickname.as_str() {
                Some(s) => Ok(s.to_string()),
                None => Err(())
            },
            None => Err(())
        },
        None => Err(())
    }
}

pub async fn get_nickname_by_id(user_id: String, implements: &Vec<String>) -> String {
    let re = Regex::new(r#"\d+"#).expect("Failed to build regex");
    if !re.is_match(&user_id) {
        return user_id;
    }
    let mut nickname = "未知".to_string();
    for implement in implements {
        nickname = match get_nickname(&user_id, implement).await {
            Ok(nick) => nick,
            Err(_) => continue
        };
        break;
    }
    debug!("Nickname: {}", nickname);
    nickname
}