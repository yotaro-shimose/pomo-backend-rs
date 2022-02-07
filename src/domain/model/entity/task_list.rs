use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskList {
    pub id: String,
    pub name: String,
}
impl TaskList {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
