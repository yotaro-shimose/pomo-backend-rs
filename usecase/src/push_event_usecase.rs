use actix_web::Result;
use domain::model::value::{Event, UserId};
use domain::repository::{DBRepository, GoogleRepository};

pub async fn push_event_usecase(
    id: &UserId,
    event: Event,
    google_repository: &impl GoogleRepository,
    db_repository: &impl DBRepository,
) -> Result<()> {
    let user = db_repository.retrieve_user(id).await?;
    let user_config = user.try_get_user_config()?;
    google_repository
        .push_event(event, &user.token, &user_config.calendar_id)
        .await?;
    Ok(())
}
