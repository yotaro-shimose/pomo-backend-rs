use crate::reqwest_google_repository::domain::model::entity::Credential;
use crate::reqwest_google_repository::domain::service::{RequestAndParse, WithCredential};
use actix_web::Result;
use domain::model::entity::TaskList;
use domain::model::value::{ClientInfo, TaskListId, Token};
use serde::{Deserialize, Serialize};

const GOOGLE_TASK_LIST_ENDPOINT: &str = "https://tasks.googleapis.com/tasks/v1/users/@me/lists";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskListQuery {
    max_results: u8,
}

impl TaskListQuery {
    fn new() -> Self {
        Self { max_results: 100 }
    }
}

#[derive(Debug, Deserialize)]
struct TaskListResponse {
    items: Vec<RawTaskList>,
}

#[derive(Debug, Deserialize)]
struct RawTaskList {
    id: String,
    title: String,
}

impl From<RawTaskList> for TaskList {
    fn from(raw: RawTaskList) -> Self {
        Self {
            id: TaskListId::new(raw.id),
            name: raw.title,
        }
    }
}

pub async fn fetch_task_list_usecase(
    token: &Token,
    client_info: &ClientInfo,
) -> Result<Vec<TaskList>> {
    let mut credential = Credential::new(token, client_info);
    let query = TaskListQuery::new();
    let builder = reqwest::Client::new()
        .get(GOOGLE_TASK_LIST_ENDPOINT)
        .query(&query)
        .with_credential(&mut credential)
        .await?;
    let response: TaskListResponse = builder.request_and_parse().await?;
    let task_lists = response
        .items
        .into_iter()
        .map(|raw| raw.into())
        .collect::<Vec<TaskList>>();
    Ok(task_lists)
}
