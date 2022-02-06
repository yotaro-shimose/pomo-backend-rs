use crate::domain::{
    model::value::{AppState, Code, UserId},
    repository::{DBRepository, GoogleRepository},
};
use crate::usecase::login_usecase;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    authorization_code: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    id: String,
}

impl LoginResponse {
    fn new(user_id: UserId) -> Self {
        Self { id: user_id.value }
    }
}

pub async fn login<G, U>(
    request: web::Json<LoginRequest>,
    state: web::Data<AppState<G, U>>,
) -> Result<HttpResponse>
where
    G: GoogleRepository,
    U: DBRepository,
{
    let code = Code::new(request.authorization_code.clone());
    let google_repository = &state.google_repository;
    let db_repository = &state.db_repository;
    let user_id = login_usecase(&code, google_repository, db_repository).await?;
    let response = LoginResponse::new(user_id);
    Ok(HttpResponse::Ok().json(response))
}
