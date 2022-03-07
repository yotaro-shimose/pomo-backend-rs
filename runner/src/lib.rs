use domain::model::value::AppState;
use infrastructure::{
    dynamo_db_repository::DynamoDBRepository, reqwest_google_repository::ReqwestGoogleRepository,
};
use presentation::lambda_server::LambdaServerError;

pub async fn load_lambda_app_state(
) -> Result<AppState<ReqwestGoogleRepository, DynamoDBRepository>, LambdaServerError> {
    let db_repository = DynamoDBRepository::new().await;
    let google_repository = ReqwestGoogleRepository::new()
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    Ok(AppState::new(google_repository, db_repository))
}

pub fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}
