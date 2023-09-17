use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub profile_pic: String
}

impl User {
    pub fn fetch_as_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
