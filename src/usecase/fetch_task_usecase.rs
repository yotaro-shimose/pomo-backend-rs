use crate::domain::model::entity::Task;
use crate::domain::{
    model::value::UserId,
    repository::{DBRepository, GoogleRepository},
};
use actix_web::Result;

pub async fn fetch_task_usecase(
    id: &UserId,
    google_repository: &impl GoogleRepository,
    db_repository: &impl DBRepository,
) -> Result<Vec<Task>> {
    let user = db_repository.retrieve_user(id).await?;
    let token = &user.token;
    let user_config = user.try_get_user_config()?;
    Ok(google_repository
        .fetch_task(token, &user_config.task_list_id)
        .await?)
}
