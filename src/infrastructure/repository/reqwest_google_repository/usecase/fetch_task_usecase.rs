use crate::{
    domain::model::value::{ClientInfo, Task, TaskListId, Token},
    infrastructure::repository::reqwest_google_repository::domain::{
        model::entity::Credential,
        service::{RequestAndParse, WithCredential},
    },
};
use actix_web::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskQuery {
    max_result: u8,
    show_completed: bool,
}

impl TaskQuery {
    fn new() -> Self {
        Self {
            max_result: 100,
            show_completed: false,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TaskResponse {
    resource: Tasks,
}

#[derive(Debug, Deserialize)]
struct Tasks {
    items: Vec<RawTask>,
}

#[derive(Debug, Deserialize)]
struct RawTask {
    id: String,
    title: String,
}

impl From<RawTask> for Task {
    fn from(raw: RawTask) -> Self {
        Self {
            id: raw.id,
            name: raw.title,
        }
    }
}

pub async fn fetch_task_usecase(
    token: &Token,
    task_list_id: &TaskListId,
    client_info: &ClientInfo,
) -> Result<Vec<Task>> {
    let mut credential = Credential::new(token, client_info);
    let endpoint = format!(
        "https://tasks.googleapis.com/tasks/v1/lists/{}/tasks",
        task_list_id.value
    );
    let query = TaskQuery::new();
    let builder = reqwest::Client::new()
        .get(endpoint)
        .query(&query)
        .with_credential(&mut credential)
        .await?;
    let response: TaskResponse = builder.request_and_parse().await?;
    let tasks = response
        .resource
        .items
        .into_iter()
        .map(|raw| raw.into())
        .collect::<Vec<Task>>();
    Ok(tasks)
}
