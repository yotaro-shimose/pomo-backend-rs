use crate::{
    domain::{FrontEndEvent, SuccessResponse},
    lambda_server::{extract_id, log_response, parse_body, LambdaServerError},
};
use domain::{
    model::value::AppState,
    repository::{DBRepository, GoogleRepository},
};
use lambda_http::{self, Error, Request, Response};
use std::sync::Arc;
use usecase::push_event_usecase;

pub async fn push_event<G, U>(req: Request) -> Result<Response<String>, Error>
where
    G: GoogleRepository + 'static,
    U: DBRepository + 'static,
{
    let ret = push_event_inner::<G, U>(req).await;
    log_response(&ret);
    ret
}

async fn push_event_inner<G, U>(req: Request) -> Result<Response<String>, Error>
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
    let body = req.body();
    let event = parse_body::<FrontEndEvent>(body)?.into();
    push_event_usecase(&id, event, google_repository, db_repository)
        .await
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    let response = SuccessResponse::new();
    let body = serde_json::to_string(&response)
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    Ok(Response::new(body))
}
