use crate::{
    domain::{LoginRequest, LoginResponse},
    lambda_server::{log_response, parse_body, LambdaServerError},
};
use domain::{
    model::value::{AppState, Code},
    repository::{DBRepository, GoogleRepository},
};
use lambda_http::{self, Error, Request, Response};
use std::sync::Arc;
use usecase::login_usecase;

pub async fn login<G, U>(req: Request) -> Result<Response<String>, Error>
where
    G: GoogleRepository + 'static,
    U: DBRepository + 'static,
{
    let ret = login_inner::<G, U>(req).await;
    log_response(&ret);
    ret
}

async fn login_inner<G, U>(req: Request) -> Result<Response<String>, Error>
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
    let body = req.body();
    let login_request = parse_body::<LoginRequest>(body)?;
    let code = Code::from(login_request);
    let user_id = login_usecase(&code, google_repository, db_repository)
        .await
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    let login_response = LoginResponse::new(user_id);
    let response_body = serde_json::to_string(&login_response)?;
    Ok(Response::new(response_body))
}
