use std::collections::HashMap;

use crate::user::User;

pub struct AccountHandler {
    pub users: HashMap<u128, User>
}
impl AccountHandler {
    pub fn new() -> AccountHandler {
        AccountHandler {
            users: HashMap::new()
        }
    }

    pub fn save(&self) {
        User::save(&self);
    }

    pub fn load() -> AccountHandler {
        let mut r = AccountHandler::new();
        r.users = User::load();

        r
    }
}
