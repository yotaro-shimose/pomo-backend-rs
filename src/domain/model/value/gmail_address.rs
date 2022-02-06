use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GmailAddress {
    pub value: String,
}

impl GmailAddress {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
