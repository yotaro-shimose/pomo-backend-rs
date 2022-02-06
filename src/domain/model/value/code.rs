use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Code {
    pub value: String,
}

impl Code {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
