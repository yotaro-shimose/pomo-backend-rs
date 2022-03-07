use super::GmailAddress;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserId {
    pub value: String,
}

impl UserId {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<GmailAddress> for UserId {
    fn from(addr: GmailAddress) -> Self {
        let id = hash_email(&addr.value);
        Self { value: id }
    }
}

impl AsRef<[u8]> for UserId {
    fn as_ref(&self) -> &[u8] {
        self.value.as_ref()
    }
}

fn hash_email(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(email);
    hasher
        .finalize()
        .to_vec()
        .iter()
        .map(|&val| val.to_string())
        .collect()
}
