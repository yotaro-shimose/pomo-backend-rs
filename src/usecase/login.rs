use crate::domain::{
    model::value::{code::Code, user_id::UserId},
    repository::GoogleRepository,
};
use actix_web::Result;
pub async fn login(code: &Code, google_repository: &impl GoogleRepository) -> Result<UserId> {
    let token = google_repository.fetch_token(code).await?;
    let email = google_repository.fetch_gmail_address(&token).await?;
    Ok(UserId::from(email))
}
