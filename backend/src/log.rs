use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub struct Entry {
    pub id: u128,
    pub species: Species,

    pub time: u128, // epoch unix utc >>>> ALWAYS EPOCH UNIX
    pub timezone_offset: i64, // offset by seconds (can be negative or positive)

    pub to: u128,
    pub to_wallet: u128,

    pub from: u128,
    pub from_wallet: u128,

    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub entries: HashMap<u128, Entry>
    // TODO: some log caching system
    // append log ids where a user is attributed with to the user itself
    // user_id -> vec<log_id>
}
impl Log {
    pub fn new() -> Log {
        Log {
            entries: HashMap::new()
        }
    }

    pub fn load() -> Log {
        serde_json::from_str(fs::read_to_string("data/logs.json").unwrap().as_str()).unwrap()
    }

    pub fn save(&self) {
        fs::write("data/logs.json", serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }

    pub fn log(&mut self, time: u128, timezone_offset: i64, species: Species, to: u128, to_wallet: u128, from: u128, from_wallet: u128, amount: f64) {
        let id = self.generate_log_id();
        self.entries.insert(id, Entry {
            id,
            species,

            time,
            timezone_offset,

            to,
            to_wallet,

            from,
            from_wallet,

            amount
        });
    }

    fn generate_log_id(&self) -> u128 {
        self.entries
            .keys()
            .max()
            .map_or(0, |i| i + 1)
    }

    pub fn get_log(&self, user_id: u128) -> Vec<Entry> {
        let mut result: Vec<Entry> = vec![];

        for (_, e) in &self.entries {
            if (e.to == user_id) || (e.from == user_id) {
                result.push(e.clone());
            }
        }

        result
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum Species {
    Transfer,
    TopUp,
}
