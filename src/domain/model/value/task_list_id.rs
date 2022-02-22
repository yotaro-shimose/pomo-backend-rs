use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TaskListId {
    pub value: String,
}
impl TaskListId {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
