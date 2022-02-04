use crate::domain::repository::{GoogleRepository, UserRepository};
use actix_web::Result;

#[derive(Debug, Clone)]
pub struct AppState<G, U>
where
    G: GoogleRepository,
    U: UserRepository,
{
    pub google_repository: G,
    pub user_repository: U,
}

impl<G, U> AppState<G, U>
where
    G: GoogleRepository,
    U: UserRepository,
{
    pub fn new(google_repository: G, user_repository: U) -> Result<Self> {
        Ok(Self {
            google_repository,
            user_repository,
        })
    }
}
