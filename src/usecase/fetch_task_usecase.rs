use crate::domain::{
    model::value::{Task, UserId},
    repository::{DBRepository, GoogleRepository},
};
use actix_web::{error, Result};

pub async fn fetch_task_usecase(
    id: &UserId,
    google_repository: &impl GoogleRepository,
    db_repository: &impl DBRepository,
) -> Result<Vec<Task>> {
    let user = db_repository
        .fetch_user(id)?
        .ok_or_else(|| error::ErrorNotFound(format!("No User Matched Id {}", id)))?;
    let token = user.token;
    let task_list_id = user.task_list_id.ok_or_else(|| {
        error::ErrorNotFound(format!(
            "TaskListId Is Not Configured For User With User Id {}",
            user.id
        ))
    })?;
    Ok(google_repository.fetch_task(&token, &task_list_id).await?)
}
