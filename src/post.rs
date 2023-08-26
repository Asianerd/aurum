use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub images: Vec<String>
}

impl Post {
    pub fn fetch_as_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
