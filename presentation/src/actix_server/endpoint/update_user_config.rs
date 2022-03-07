use crate::actix_server::IdHeader;
use crate::domain::{FrontEndUserConfig, SuccessResponse};
use actix_web::{error, web, HttpResponse, Result};
use domain::{
    model::value::{AppState, UserConfig, UserId},
    repository::{DBRepository, GoogleRepository},
};
use std::convert::Into;
use usecase::update_user_config_usecase;

pub async fn update_user_config<G, U>(
    id_header: IdHeader,
    raw_config: web::Json<FrontEndUserConfig>,
    state: web::Data<AppState<G, U>>,
) -> Result<HttpResponse>
where
    G: GoogleRepository,
    U: DBRepository,
{
    let id = UserId::new(id_header.id);
    let db_repository = &state.db_repository;
    let user_config = Into::<Option<UserConfig>>::into(raw_config.0)
        .ok_or_else(|| error::ErrorBadRequest("No User Config Specified"))?;
    update_user_config_usecase(&id, user_config, db_repository).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::new()))
}
