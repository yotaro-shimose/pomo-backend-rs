use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CalendarId {
    pub value: String,
}
impl CalendarId {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
