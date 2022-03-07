use crate::repository::{DBRepository, GoogleRepository};

#[derive(Debug, Clone)]
pub struct AppState<G, U>
where
    G: GoogleRepository + 'static,
    U: DBRepository + 'static,
{
    pub google_repository: G,
    pub db_repository: U,
}

impl<G, U> AppState<G, U>
where
    G: GoogleRepository + 'static,
    U: DBRepository + 'static,
{
    pub fn new(google_repository: G, db_repository: U) -> Self {
        Self {
            google_repository,
            db_repository,
        }
    }
}
