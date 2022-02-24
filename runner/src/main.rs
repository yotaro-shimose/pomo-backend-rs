use actix_web::Result;
use domain::model::value::AppState;
use infrastructure::reqwest_google_repository::ReqwestGoogleRepository;
use infrastructure::sled_db_repository::SledDBRepository;
use presentation::actix_server::Server;

#[actix_web::main]
async fn main() -> Result<()> {
    let google_repository = ReqwestGoogleRepository::new()?;
    let sled_db_repository = SledDBRepository::new()?;
    let app_state = AppState::new(google_repository, sled_db_repository);
    Server::run(app_state).await?;
    Ok(())
}
