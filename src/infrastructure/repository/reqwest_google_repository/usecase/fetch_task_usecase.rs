use crate::domain::model::value::{ClientInfo, Task, TaskListId, Token};
use actix_web::Result;

pub async fn fetch_task_usecase(
    token: &Token,
    task_list_id: &TaskListId,
    client_info: &ClientInfo,
) -> Result<Vec<Task>> {
    todo!();
}
