use serde::Serialize;

#[derive(Serialize)]
pub struct CaveCount {
    pub total: usize,
    pub valid: usize
}


#[derive(Serialize)]
pub struct IndexResponse {
    pub version: String,
    pub count: CaveCount,
    pub code: u16
}


#[derive(Serialize)]
pub struct Error {
    pub code: u16,
    pub message: String
}

#[derive(Serialize)]
pub struct CaveItem {
    pub code: u16,
    pub id: u64,
    pub content: String,
    pub time: f64,
    pub sender: String
}
