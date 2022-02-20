use crate::domain::{
    model::value::{UserConfig, UserId},
    repository::DBRepository,
};
use actix_web::{error, Result};

pub async fn update_user_config_usecase(
    id: &UserId,
    config: UserConfig,
    db_repository: &impl DBRepository,
) -> Result<()> {
    let mut user = db_repository
        .fetch_user(id)
        .await?
        .ok_or_else(|| error::ErrorNotFound(format!("No User Matched Id {}", id)))?;
    user.update_config(config);
    db_repository.save_user(&user).await?;
    Ok(())
}
