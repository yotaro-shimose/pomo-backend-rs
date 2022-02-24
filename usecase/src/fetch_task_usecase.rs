use actix_web::Result;
use domain::model::entity::Task;
use domain::model::value::UserId;
use domain::repository::{DBRepository, GoogleRepository};

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
