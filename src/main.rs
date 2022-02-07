use actix_web::Result;
use backend::domain::model::value::AppState;
use backend::infrastructure::reqwest_google_repository::ReqwestGoogleRepository;
use backend::infrastructure::sled_db_repository::SledDBRepository;
use backend::presentation::Server;

#[actix_web::main]
async fn main() -> Result<()> {
    let google_repository = ReqwestGoogleRepository::new()?;
    let sled_db_repository = SledDBRepository::new()?;
    let app_state = AppState::new(google_repository, sled_db_repository);
    Server::run(app_state).await?;
    Ok(())
}
