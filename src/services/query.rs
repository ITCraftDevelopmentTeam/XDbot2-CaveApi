use serde::Deserialize;

#[derive(Deserialize)]
pub struct Random {
    pub no_image: Option<bool>,
    pub max_length: Option<usize>,
    pub ret: Option<String>,
    pub include_comments: Option<bool>
}