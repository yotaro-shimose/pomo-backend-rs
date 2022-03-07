use crate::{
    domain::FrontEndCalendar,
    lambda_server::{extract_id, log_response, LambdaServerError},
};
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
    let ret = fetch_calendar_inner::<G, U>(req).await;
    log_response(&ret);
    ret
}

async fn fetch_calendar_inner<G, U>(req: Request) -> Result<Response<String>, Error>
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
        .map_err(|err| err.to_string())?
        .into_iter()
        .map(|val| val.into())
        .collect::<Vec<FrontEndCalendar>>();
    let body = serde_json::to_string(&calendars)
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    Ok(Response::new(body))
}
