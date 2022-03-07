use super::usecase::{
    fetch_calendar_usecase, fetch_gmail_address_usecase, fetch_task_list_usecase,
    fetch_task_usecase, fetch_token_usecase, push_event_usecase,
};

use domain::model::entity::{Calendar, Task, TaskList};
use domain::{
    model::value::{CalendarId, ClientInfo, Code, Event, GmailAddress, TaskListId, Token},
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

    async fn fetch_calendar(&self, token: &Token) -> Result<Vec<Calendar>> {
        let calendars = fetch_calendar_usecase(token, &self.client_info).await?;
        Ok(calendars)
    }

    async fn push_event(
        &self,
        event: Event,
        token: &Token,
        calendar_id: &CalendarId,
    ) -> Result<()> {
        push_event_usecase(event, token, calendar_id, &self.client_info).await?;
        Ok(())
    }
}
