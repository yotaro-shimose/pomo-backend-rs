use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskListId {
    value: String,
}
impl TaskListId {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
