use super::extract_id;
use crate::lambda_server::LambdaServerError;
use domain::{
    model::value::AppState,
    repository::{DBRepository, GoogleRepository},
};
use lambda_http::{self, Error, Request, Response};
use std::sync::Arc;
use usecase::fetch_calendar_usecase;

pub async fn fetch_calendar<G, U>(req: Request) -> Result<Response<String>, Error>
where
    G: GoogleRepository + 'static,
    U: DBRepository + 'static,
{
    let state = req
        .extensions()
        .get::<Arc<AppState<G, U>>>()
        .ok_or_else(|| {
            LambdaServerError::InternalServerError("AppState Is Not Properly Set".to_string())
        })?;
    let google_repository = &state.google_repository;
    let db_repository = &state.db_repository;
    let id = extract_id(&req)?;
    let calendars = fetch_calendar_usecase(&id, google_repository, db_repository)
        .await
        .map_err(|err| err.to_string())?;
    let body = serde_json::to_string(&calendars)
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    Ok(Response::new(body))
}
