use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarId {
    value: String,
}
impl CalendarId {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
