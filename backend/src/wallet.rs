use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    // per user id
    pub id: u128, // 0 -> main wallet
    pub name: String,
    pub balance: f64,
    pub colour: u128,


    pub limit: (f64, f64) // dont allow anymore/less than this
    // (0, 0) means no limit whatsoever
}
impl Wallet {
    pub fn default_wallet() -> Wallet {
        // returns empty wallet with id 0 and the name wallet
        Wallet {
            id: 0,
            name: "wallet".to_string(),
            balance: 0f64,
            colour: 0,
            limit: (0f64, 0f64)
        }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn alter_balance(&mut self, amount: f64) -> WalletResult {
        let mut n = self.balance.clone();
        n += amount;
        if n < 0f64 {
            return WalletResult::InsufficientAmount;
        }
        self.balance = n;
        WalletResult::Success
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub enum WalletResult {
    Success,

    WalletNoExist,

    InsufficientAmount
}

// implement other currencies?
