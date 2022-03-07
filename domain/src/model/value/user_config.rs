use crate::model::value::{CalendarId, TaskListId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct UserConfig {
    pub task_list_id: TaskListId,
    pub calendar_id: CalendarId,
}

impl UserConfig {
    pub fn new(task_list_id: TaskListId, calendar_id: CalendarId) -> Self {
        Self {
            task_list_id,
            calendar_id,
        }
    }
}
