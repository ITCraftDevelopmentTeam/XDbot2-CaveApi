mod sender;

use serde::Deserialize;
use regex::Regex;
use serde_json::{Value, Map};
use rand::seq::SliceRandom;
use log::{error, warn};
use std::collections::HashMap;
// use std::io::Read;
use tokio::fs;
use std::path::PathBuf;
use base64::Engine;

// use std::fs::File;

pub struct CaveCount {
    pub total: usize,
    pub valid: usize
}

#[derive(Deserialize)]
pub struct CaveData {
    pub count: usize,
    pub data: Map<String, Value>
}


pub struct CaveItemData {
    pub id: u64,
    pub content: String,
    pub sender: String,
    pub time: f64,
    pub images: HashMap<String, Option<String>>
}

pub struct DataHelper {
    pub base_path: PathBuf,
    pub implements: Vec<String>
}


impl DataHelper {

    async fn load_cave_data(&self) -> Result<CaveData, String> {
        let path = self.base_path.join("cave.data.json");
        // let mut file = match File::open(path.clone()) {
        //     Ok(f) => f,
        //     Err(err) => {
        //         error!("Failed to open {}: {}", path.display(), err);
        //         return Err(err.to_string());
        //     }
        // };
        // let mut json_data = String::new();
        // match file.read_to_string(&mut json_data) {
        //     Ok(_) => {},
        //     Err(err) => {
        //         error!("Failed to read file {}: {}", path.display(), err);
        //         return Err(err.to_string());
        //     }
        // }
        let json_data = match fs::read_to_string(&path).await {
            Ok(data) => data,
            Err(err) => {
                error!("Failed to read file {}: {}", path.display(), err);
                return Err(err.to_string());
            }
        };
        match serde_json::from_str(&json_data) {
            Ok(data) => Ok(data),
            Err(err) => {
                error!("Failed to load json {}: {}", path.display(), err);
                return Err(err.to_string());
            }
        }
    }

    pub async fn get_cave_count(&self) -> Result<CaveCount, String> {
        let data = match self.load_cave_data().await {
            Ok(value) => value,
            Err(e) => {return Err(e);}
        };
        Ok(CaveCount {
            total: data.count,
            valid: data.data.keys().count()
        })
    }
    

    async fn get_cave_list(&self, max_length: usize, no_image: bool) -> Result<Vec<CaveItemData>, String> {
        let original_data = match self.load_cave_data().await {
            Ok(data) => data.data,
            Err(err) => return Err(err)
        };
        let mut cave_list: Vec<CaveItemData> = Vec::new();
        for (_key, value) in &original_data {
            let data = parse_cave_data(value);
            if check_cave(&data, max_length, no_image) {
                cave_list.push(data);
            }
        }
        Ok(cave_list)
    }

    async fn get_images(&self, content: String) -> HashMap<String, Option<String>> {
        let mut images: HashMap<String, Option<String>> = HashMap::new();
        for image_id in get_all_images(content) {
            images.insert(image_id.clone(), self.get_image(image_id).await);
        }
        images
    }

    pub async fn get_image(&self, image_id: String) -> Option<String> {
        let path: PathBuf = self.base_path.join(format!("caveImages/{}.png", image_id));
        // let mut file: File = match File::open(path) {
        //     Ok(ret) => ret,
        //     Err(err) => {
        //         warn!("Failed to open {}: {}", image_id, err.to_string());
        //         return None
        //     }
        // };
        // let mut buffer = Vec::new();
        // if let Err(err) = file.read_to_end(&mut buffer) {
        //     warn!("Failed to read {}: {}", image_id, err.to_string());
        //     return None
        // }
        let buffer = match fs::read(&path).await {
            Ok(buf) => buf,
            Err(err) => {
                warn!("Failed to read {}: {}", image_id, err.to_string());
                return None
            }
        };
        let engine = base64::engine::general_purpose::STANDARD_NO_PAD;
        let base64_string = engine.encode(&buffer);
        
        Some(base64_string.to_string())

    }

    pub async fn random_cave(&self, max_length: usize, no_image: bool) -> Result<CaveItemData, String> {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let cave_list: Vec<CaveItemData> = match self.get_cave_list(max_length, no_image).await {
            Ok(list) => list,
            Err(err) => return Err(err)
        };
        
        match cave_list.choose(&mut rng) {
            Some(item) => Ok(CaveItemData {
                id: item.id,
                content: item.content.clone(),
                sender: sender::get_nickname_by_id(item.sender.clone(), &self.implements).await,
                time: item.time.clone(),
                images: self.get_images(item.content.clone()).await
            }),
            None => Err("没有符合要求的回声洞".to_string())
        }
    }

}

fn get_json_value(json: &Value, key: &str, default: &str) -> String {
    match json.get(key) {
        Some(value) => value.as_str().unwrap_or(default).to_string(),
        None => default.to_string()
    }
}

fn parse_cave_data(json_data: &Value) -> CaveItemData {
    let content = get_json_value(json_data, "text", "");
    CaveItemData {
        id: match json_data.get("id") {
            Some(value) => value.as_u64().unwrap_or(std::u64::MAX),
            None => std::u64::MAX
        },
        content: content.clone(),
        sender: get_cave_sender(json_data.get("sender")),
        time: match json_data.get("time") {
            Some(value) => value.as_f64().unwrap_or(0.0),
            None => 0.0
        },
        images: HashMap::new()
    }
}

fn check_cave(cave: &CaveItemData, max_length: usize, no_image: bool) -> bool {
    if cave.content.contains("[[Img:") && no_image {
        false
    } else if cave.content.len() > max_length {
        false
    } else {
        true
    }
}

fn get_all_images(content: String) -> Vec<String> {
    let re = Regex::new(r#"\[\[Img:\d+\.\d+\]\]\]"#);
    let mut images = Vec::new();
    for mat in re.expect("Failed to init Regex").find_iter(&content) {
        let s = mat.as_str().to_string();
        images.push((&s[6..(s.len() - 3)]).to_string());
    }
    return images
}

fn get_cave_sender(json_data: Option<&Value>) -> String {
    let data = match json_data {
        Some(value) => value,
        None => return "未知".to_string()
    };
    if data.is_object() {
        get_json_value(data, "name", "未知")
    } else {
        data.as_str().unwrap_or("未知").to_string()
    }
}