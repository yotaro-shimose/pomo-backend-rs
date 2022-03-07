use crate::domain::FrontEndUserConfig;

use crate::actix_server::IdHeader;
use actix_web::{web, HttpResponse, Result};
use domain::{
    model::value::{AppState, UserId},
    repository::{DBRepository, GoogleRepository},
};
use usecase::fetch_user_config_usecase;

pub async fn fetch_user_config<G, U>(
    id_header: IdHeader,
    state: web::Data<AppState<G, U>>,
) -> Result<HttpResponse>
where
    G: GoogleRepository,
    U: DBRepository,
{
    let id = UserId::new(id_header.id);
    let db_repository = &state.db_repository;
    let user_config = fetch_user_config_usecase(&id, db_repository).await?;
    let response: FrontEndUserConfig = user_config.into();
    Ok(HttpResponse::Ok().json(response))
}
