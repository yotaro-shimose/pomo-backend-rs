use super::IdHeader;
use crate::domain::{
    model::value::{AppState, UserConfig, UserId},
    repository::{DBRepository, GoogleRepository},
};
use crate::usecase::fetch_user_config_usecase;
use actix_web::{web, HttpResponse, Result};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FrontEndUserConfig {
    task_list_id: Option<String>,
    calendar_id: Option<String>,
}

impl From<Option<UserConfig>> for FrontEndUserConfig {
    fn from(user_config: Option<UserConfig>) -> Self {
        match user_config {
            Some(user_config) => Self {
                task_list_id: Some(user_config.task_list_id.value),
                calendar_id: Some(user_config.calendar_id.value),
            },
            None => Self {
                task_list_id: None,
                calendar_id: None,
            },
        }
    }
}

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
