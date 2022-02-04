use super::GmailAddress;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserId {
    pub value: String,
}

impl From<GmailAddress> for UserId {
    fn from(addr: GmailAddress) -> Self {
        let id = hash_email(&addr.value);
        Self { value: id }
    }
}

fn hash_email(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(email);
    hasher
        .finalize()
        .to_vec()
        .iter()
        .map(|&val| val as char)
        .collect()
}
