use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
}
impl Task {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
