use crate::domain::{LoginRequest, LoginResponse};
use actix_web::{web, HttpResponse, Result};
use domain::{
    model::value::{AppState, Code},
    repository::{DBRepository, GoogleRepository},
};
use usecase::login_usecase;

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
