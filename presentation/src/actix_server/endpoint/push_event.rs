use crate::actix_server::IdHeader;
use crate::domain::{FrontEndEvent, SuccessResponse};
use actix_web::{web, HttpResponse, Result};
use domain::model::value::{AppState, UserId};
use domain::repository::{DBRepository, GoogleRepository};
use usecase::push_event_usecase;

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
