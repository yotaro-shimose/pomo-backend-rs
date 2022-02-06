use crate::domain::{
    model::value::{UserConfig, UserId},
    repository::DBRepository,
};
use actix_web::{error, Result};

pub async fn fetch_user_config_usecase(
    id: &UserId,
    db_repository: &impl DBRepository,
) -> Result<Option<UserConfig>> {
    let user = db_repository
        .fetch_user(id)?
        .ok_or_else(|| error::ErrorNotFound(format!("No User Matched Id {}", id)))?;
    let config = user.user_config;
    Ok(config)
}
