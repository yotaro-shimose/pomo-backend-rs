use crate::domain::model::value::{Token, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub token: Token,
    pub calendar_id: Option<String>,
    pub task_list_id: Option<String>,
}

impl User {
    pub fn new(
        id: UserId,
        token: Token,
        calendar_id: Option<String>,
        task_list_id: Option<String>,
    ) -> Self {
        Self {
            id,
            token,
            calendar_id,
            task_list_id,
        }
    }
}
