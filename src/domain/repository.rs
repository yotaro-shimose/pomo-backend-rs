use super::model::{
    entity::{Calendar, Task, TaskList, User},
    value::{CalendarId, Event, GmailAddress, TaskListId, UserId},
};
use crate::domain::model::value::{Code, Token};
use actix_web::{error, Result};
use async_trait::async_trait;

#[async_trait]
pub trait GoogleRepository: Send + Sync + Clone {
    async fn fetch_token(&self, code: &Code) -> Result<Token>;
    async fn fetch_gmail_address(&self, token: &Token) -> Result<GmailAddress>;
    async fn fetch_task(&self, token: &Token, task_list_id: &TaskListId) -> Result<Vec<Task>>;
    async fn fetch_task_list(&self, token: &Token) -> Result<Vec<TaskList>>;
    async fn fetch_calendar(&self, token: &Token) -> Result<Vec<Calendar>>;
    async fn push_event(&self, event: Event, token: &Token, calendar_id: &CalendarId)
        -> Result<()>;
}

#[async_trait]
pub trait DBRepository: Send + Sync + Clone {
    async fn fetch_user(&self, id: &UserId) -> Result<Option<User>>;
    async fn save_user(&self, user: &User) -> Result<()>;
    async fn retrieve_user(&self, id: &UserId) -> Result<User> {
        self.fetch_user(id)
            .await?
            .ok_or_else(|| error::ErrorNotFound(format!("No User Matched Id {}", id)))
    }
    async fn delete_user(&self, id: &UserId) -> Result<()>;
}
