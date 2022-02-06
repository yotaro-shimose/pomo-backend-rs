use crate::domain::model::value::{CalendarId, TaskListId, Token, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub token: Token,
    pub calendar_id: Option<CalendarId>,
    pub task_list_id: Option<TaskListId>,
}

impl User {
    pub fn new(
        id: UserId,
        token: Token,
        calendar_id: Option<CalendarId>,
        task_list_id: Option<TaskListId>,
    ) -> Self {
        Self {
            id,
            token,
            calendar_id,
            task_list_id,
        }
    }
}
