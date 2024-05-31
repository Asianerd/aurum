// let soterius: HashMap<String, (String, u128)> = serde_json::from_str(fs::read_to_string("../../data/users.json").unwrap().as_str()).unwrap();
// println!("{:?}", soterius);

// match soterius.get(&self.username) {
//     Some((password, user_id)) => {
//         if *password != *self.password {
//             return LoginResult::PasswordWrong;
//         }

//         let aurum_lookup = User::lookup_user_id(account_handler, &self.username);
//         if aurum_lookup.is_none() {
//             account_handler.users.insert(*user_id, User {
//                 id: *user_id,
//                 username: self.username.clone(),
//                 wallets: HashMap::from([(0u128, Wallet::default_wallet())])
//             });
//         }

//         return LoginResult::Success(*user_id);
//     },
//     None => LoginResult::UsernameNoExist
// }

use std::{collections::HashMap, fs};

pub fn fetch(username: String) -> Option<(u128, String)> {
    let soterius: HashMap<String, (u128, String)> = serde_json::from_str(fs::read_to_string("../../data/users.json").unwrap().as_str()).unwrap();
    soterius.get(&username).map_or_else(|| None, |e| Some((e.0.clone(), e.1.clone())))
}
