use super::model::{
    entity::{Task, TaskList, User},
    value::{GmailAddress, TaskListId, UserId},
};
use crate::domain::model::value::{Code, Token};
use actix_web::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GoogleRepository: Send + Sync + Clone {
    async fn fetch_token(&self, code: &Code) -> Result<Token>;
    async fn fetch_gmail_address(&self, token: &Token) -> Result<GmailAddress>;
    async fn fetch_task(&self, token: &Token, task_list_id: &TaskListId) -> Result<Vec<Task>>;
    async fn fetch_task_list(&self, token: &Token) -> Result<Vec<TaskList>>;
}

pub trait DBRepository: Send + Sync + Clone {
    fn fetch_user(&self, id: &UserId) -> Result<Option<User>>;
    fn save_user(&self, user: &User) -> Result<()>;
}
