use super::super::SuccessResponse;
use super::IdHeader;
use actix_web::{error, web, HttpResponse, Result};
use domain::{
    model::value::{AppState, CalendarId, TaskListId, UserConfig, UserId},
    repository::{DBRepository, GoogleRepository},
};
use serde::Deserialize;
use usecase::update_user_config_usecase;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontEndUserConfig {
    task_list_id: Option<String>,
    calendar_id: Option<String>,
}

fn try_convert(raw: FrontEndUserConfig) -> Option<UserConfig> {
    if let (Some(task_list_id), Some(calendar_id)) = (raw.task_list_id, raw.calendar_id) {
        let task_list_id = TaskListId::new(task_list_id);
        let calendar_id = CalendarId::new(calendar_id);
        let user_config = UserConfig::new(task_list_id, calendar_id);
        Some(user_config)
    } else {
        None
    }
}

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
    let user_config = try_convert(raw_config.0)
        .ok_or_else(|| error::ErrorBadRequest("No User Config Specified"))?;
    update_user_config_usecase(&id, user_config, db_repository).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::new()))
}
