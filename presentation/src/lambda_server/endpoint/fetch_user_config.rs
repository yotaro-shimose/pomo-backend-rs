use crate::{
    domain::FrontEndUserConfig,
    lambda_server::{extract_id, log_response, LambdaServerError},
};
use domain::{
    model::value::AppState,
    repository::{DBRepository, GoogleRepository},
};
use lambda_http::{self, Error, Request, Response};
use std::sync::Arc;
use usecase::fetch_user_config_usecase;

pub async fn fetch_user_config<G, U>(req: Request) -> Result<Response<String>, Error>
where
    G: GoogleRepository + 'static,
    U: DBRepository + 'static,
{
    let ret = fetch_user_config_inner::<G, U>(req).await;
    log_response(&ret);
    ret
}

async fn fetch_user_config_inner<G, U>(req: Request) -> Result<Response<String>, Error>
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
    let db_repository = &state.db_repository;
    let id = extract_id(&req)?;
    let user_config = fetch_user_config_usecase(&id, db_repository)
        .await
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    let response: FrontEndUserConfig = user_config.into();

    let body = serde_json::to_string(&response)
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    Ok(Response::new(body))
}
