use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expiry_date: DateTime<Utc>,
}

impl Token {
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now < self.expiry_date
    }

    pub fn update_token(&mut self, access_token: String, expiry_date: DateTime<Utc>) {
        self.access_token = access_token;
        self.expiry_date = expiry_date;
    }
}
