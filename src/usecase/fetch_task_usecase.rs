use crate::domain::{
    model::value::{Task, UserId},
    repository::{DBRepository, GoogleRepository},
};
use actix_web::Result;

pub async fn fetch_task_usecase(
    id: &UserId,
    google_repository: &impl GoogleRepository,
    db_repository: &impl DBRepository,
) -> Result<Vec<Task>> {
    todo!();
}
