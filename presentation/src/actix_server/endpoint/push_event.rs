use super::super::SuccessResponse;
use super::IdHeader;
use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, Local, TimeZone, Utc};
use domain::model::entity::Task;
use domain::model::value::{AppState, Event, UserId};
use domain::repository::{DBRepository, GoogleRepository};
use serde::{Deserialize, Deserializer};
use usecase::push_event_usecase;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontEndEvent {
    task: Task,
    #[serde(deserialize_with = "format_frontend_datetime")]
    start_time: DateTime<Utc>,
    #[serde(deserialize_with = "format_frontend_datetime")]
    end_time: DateTime<Utc>,
}

fn format_frontend_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_time: String = Deserialize::deserialize(deserializer)?;
    let local = Local
        .datetime_from_str(&string_time, "%Y-%m-%d %H:%M:%S")
        .map_err(serde::de::Error::custom)?;
    Ok(local.into())
}

impl From<FrontEndEvent> for Event {
    fn from(raw: FrontEndEvent) -> Self {
        Self {
            task: raw.task,
            start: raw.start_time,
            end: raw.end_time,
        }
    }
}

pub async fn push_event<G, U>(
    id_header: IdHeader,
    raw_event: web::Json<FrontEndEvent>,
    state: web::Data<AppState<G, U>>,
) -> Result<HttpResponse>
where
    G: GoogleRepository,
    U: DBRepository,
{
    let id = UserId::new(id_header.id);
    let event = raw_event.0.into();
    let google_repository = &state.google_repository;
    let db_repository = &state.db_repository;
    push_event_usecase(&id, event, google_repository, db_repository).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::new()))
}
