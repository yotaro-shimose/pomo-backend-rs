use serde::{Deserialize, Serialize};

use crate::model::value::TaskListId;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskList {
    pub id: TaskListId,
    pub name: String,
}
impl TaskList {
    pub fn new(id: TaskListId, name: String) -> Self {
        Self { id, name }
    }
}
