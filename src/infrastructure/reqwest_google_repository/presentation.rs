use super::usecase::{
    fetch_gmail_address_usecase, fetch_task_list_usecase, fetch_task_usecase, fetch_token_usecase,
};

use crate::domain::model::entity::{Task, TaskList};
use crate::domain::{
    model::value::{ClientInfo, Code, GmailAddress, TaskListId, Token},
    repository::GoogleRepository,
};
use actix_web::Result;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct ReqwestGoogleRepository {
    client_info: ClientInfo,
}

impl ReqwestGoogleRepository {
    pub fn new() -> Result<Self> {
        let client_info = ClientInfo::load()?;
        Ok(Self { client_info })
    }
}

#[async_trait]
impl GoogleRepository for ReqwestGoogleRepository {
    async fn fetch_token(&self, code: &Code) -> Result<Token> {
        let token = fetch_token_usecase(code, &self.client_info).await?;
        Ok(token)
    }
    async fn fetch_gmail_address(&self, token: &Token) -> Result<GmailAddress> {
        let gmail_address = fetch_gmail_address_usecase(token, &self.client_info).await?;
        Ok(gmail_address)
    }

    async fn fetch_task(&self, token: &Token, task_list_id: &TaskListId) -> Result<Vec<Task>> {
        let tasks = fetch_task_usecase(token, task_list_id, &self.client_info).await?;
        Ok(tasks)
    }

    async fn fetch_task_list(&self, token: &Token) -> Result<Vec<TaskList>> {
        let task_lists = fetch_task_list_usecase(token, &self.client_info).await?;
        Ok(task_lists)
    }
}
