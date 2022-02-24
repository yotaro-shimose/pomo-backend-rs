use actix_web::Result;
use domain::model::entity::User;
use domain::model::value::{Code, UserId};
use domain::repository::{DBRepository, GoogleRepository};

pub async fn login_usecase(
    code: &Code,
    google_repository: &impl GoogleRepository,
    db_repository: &impl DBRepository,
) -> Result<UserId> {
    let token = google_repository.fetch_token(code).await?;
    let email = google_repository.fetch_gmail_address(&token).await?;
    let id = UserId::from(email);
    let user = db_repository.fetch_user(&id).await?;
    if let Some(mut user) = user {
        user.update_token(token);
        db_repository.save_user(&user).await?;
    } else {
        let user = User::new(id.clone(), token, None);
        db_repository.save_user(&user).await?;
    }
    Ok(id)
}
