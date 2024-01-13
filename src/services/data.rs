use serde::Deserialize;
use serde_json::{Value, Map};
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

}