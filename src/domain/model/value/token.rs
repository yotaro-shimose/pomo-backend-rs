use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expiry_date: DateTime<Utc>,
}

impl Token {
    pub fn new(access_token: &str, refresh_token: &str, expiry_date: DateTime<Utc>) -> Self {
        Self {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            expiry_date,
        }
    }
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now < self.expiry_date
    }
}
