use serde::{Deserialize, Serialize};

use crate::model::value::CalendarId;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    pub id: CalendarId,
    pub name: String,
}
impl Calendar {
    pub fn new(id: CalendarId, name: String) -> Self {
        Self { id, name }
    }
}
