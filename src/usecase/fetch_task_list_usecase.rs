use crate::domain::model::entity::TaskList;
use crate::domain::{
    model::value::UserId,
    repository::{DBRepository, GoogleRepository},
};
use actix_web::{error, Result};

pub async fn fetch_task_list_usecase(
    id: &UserId,
    google_repository: &impl GoogleRepository,
    db_repository: &impl DBRepository,
) -> Result<Vec<TaskList>> {
    let user = db_repository
        .fetch_user(id)?
        .ok_or_else(|| error::ErrorNotFound(format!("No User Matched Id {}", id)))?;
    let token = &user.token;
    Ok(google_repository.fetch_task_list(token).await?)
}
