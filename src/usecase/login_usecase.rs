use crate::domain::model::entity::User;
use crate::domain::model::value::{Code, UserId};
use crate::domain::repository::{DBRepository, GoogleRepository};
use actix_web::Result;

pub async fn login_usecase(
    code: &Code,
    google_repository: &impl GoogleRepository,
    db_repository: &impl DBRepository,
) -> Result<UserId> {
    let token = google_repository.fetch_token(code).await?;
    let email = google_repository.fetch_gmail_address(&token).await?;
    let id = UserId::from(email);
    let user = db_repository.fetch_user(&id)?;
    if let Some(mut user) = user {
        user.update_token(token);
        db_repository.save_user(&user)?;
    } else {
        let user = User::new(id.clone(), token, None);
        db_repository.save_user(&user)?;
    }
    Ok(id)
}
