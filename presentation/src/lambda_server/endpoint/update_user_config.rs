use crate::{
    domain::{FrontEndUserConfig, SuccessResponse},
    lambda_server::{extract_id, log_response, parse_body, LambdaServerError},
};
use domain::{
    model::value::{AppState, UserConfig},
    repository::{DBRepository, GoogleRepository},
};
use lambda_http::{self, Error, Request, Response};
use std::sync::Arc;
use usecase::update_user_config_usecase;

pub async fn update_user_config<G, U>(req: Request) -> Result<Response<String>, Error>
where
    G: GoogleRepository + 'static,
    U: DBRepository + 'static,
{
    let ret = update_user_config_inner::<G, U>(req).await;
    log_response(&ret);
    ret
}

async fn update_user_config_inner<G, U>(req: Request) -> Result<Response<String>, Error>
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
    let body = req.body();
    let user_config = Into::<Option<UserConfig>>::into(parse_body::<FrontEndUserConfig>(body)?)
        .ok_or_else(|| LambdaServerError::BadRequest("No User Config Specified".to_string()))?;
    update_user_config_usecase(&id, user_config, db_repository)
        .await
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    let response = SuccessResponse::new();
    let body = serde_json::to_string(&response)
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    Ok(Response::new(body))
}
