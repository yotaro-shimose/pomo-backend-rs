use crate::domain::FrontEndCalendar;

use super::IdHeader;
use actix_web::{web, HttpResponse, Result};
use domain::{
    model::value::{AppState, UserId},
    repository::{DBRepository, GoogleRepository},
};
use usecase::fetch_calendar_usecase;

pub async fn fetch_calendar<G, U>(
    id_header: IdHeader,
    state: web::Data<AppState<G, U>>,
) -> Result<HttpResponse>
where
    G: GoogleRepository,
    U: DBRepository,
{
    let id = UserId::new(id_header.id);
    let google_repository = &state.google_repository;
    let db_repository = &state.db_repository;
    let response = fetch_calendar_usecase(&id, google_repository, db_repository)
        .await?
        .into_iter()
        .map(|val| val.into())
        .collect::<Vec<FrontEndCalendar>>();
    Ok(HttpResponse::Ok().json(response))
}
