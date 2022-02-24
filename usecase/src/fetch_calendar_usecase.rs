use actix_web::Result;
use domain::model::entity::Calendar;
use domain::{
    model::value::UserId,
    repository::{DBRepository, GoogleRepository},
};

pub async fn fetch_calendar_usecase(
    id: &UserId,
    google_repository: &impl GoogleRepository,
    db_repository: &impl DBRepository,
) -> Result<Vec<Calendar>> {
    let user = db_repository.retrieve_user(id).await?;
    let token = &user.token;
    Ok(google_repository.fetch_calendar(token).await?)
}
