use crate::domain::{
    model::value::AppState,
    repository::{GoogleRepository, UserRepository},
};
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
    fn new(id: String) -> Self {
        Self { id }
    }
}

pub async fn login<G, U>(
    request: web::Json<LoginRequest>,
    state: web::Data<AppState<G, U>>,
) -> Result<HttpResponse>
where
    G: GoogleRepository,
    U: UserRepository,
{
    todo!()
}
