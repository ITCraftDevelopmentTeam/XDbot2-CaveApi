use serde::Deserialize;
use serde_json::{Value, Map};
use rand::seq::SliceRandom;
use log::error;
use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

pub struct CaveCount {
    pub total: u64,
    pub valid: usize
}

#[derive(Deserialize)]
pub struct CaveData {
    pub count: u64,
    pub data: Map<String, Value>
}


pub struct CaveItemData {
    pub id: u64,
    pub content: String,
    pub sender: String
}

pub struct DataHelper {
    pub base_path: PathBuf
}


impl DataHelper {

    fn load_cave_data(&self) -> Result<CaveData, String> {
        let path = self.base_path.join("cave.data.json");
        let mut file = match File::open(path.clone()) {
            Ok(f) => f,
            Err(err) => {
                error!("Failed to open {}: {}", path.display(), err);
                return Err(err.to_string());
            }
        };
        let mut json_data = String::new();
        match file.read_to_string(&mut json_data) {
            Ok(_) => {},
            Err(err) => {
                error!("Failed to read file {}: {}", path.display(), err);
                return Err(err.to_string());
            }
        }
        match serde_json::from_str(&json_data) {
            Ok(data) => Ok(data),
            Err(err) => {
                error!("Failed to load json {}: {}", path.display(), err);
                return Err(err.to_string());
            }
        }
    }

    pub fn get_cave_count(&self) -> Result<CaveCount, String> {
        let data = match self.load_cave_data() {
            Ok(value) => value,
            Err(e) => {return Err(e);}
        };
        Ok(CaveCount {
            total: data.count,
            valid: data.data.keys().count()
        })
    }

    fn get_cave_list(&self, max_length: usize, no_image: bool) -> Result<Vec<CaveItemData>, String> {
        let original_data = match self.load_cave_data() {
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

    pub fn random_cave(&self, max_length: usize, no_image: bool) -> Result<CaveItemData, String> {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let cave_list: Vec<CaveItemData> = match self.get_cave_list(max_length, no_image) {
            Ok(list) => list,
            Err(err) => return Err(err)
        };
        
        match cave_list.choose(&mut rng) {
            Some(item) => Ok(CaveItemData { id: item.id, content: item.content.clone(), sender: item.sender.clone() }),
            None => Err("没有符合要求的回声洞".to_string())
        }
    }

}

fn get_json_value(json: &Value, key: &'static str, default: &'static str) -> String {
    match json.get(key) {
        Some(value) => value.as_str().unwrap_or(default).to_string(),
        None => default.to_string()
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

fn parse_cave_data(json_data: &Value) -> CaveItemData {
    CaveItemData {
        id: match json_data.get("id") {
            Some(value) => value.as_u64().unwrap_or(std::u64::MAX),
            None => std::u64::MAX
        },
        content: get_json_value(json_data, "text", ""),
        sender: get_cave_sender(json_data.get("sender"))
    }
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