use domain::model::value::AppState;
use infrastructure::{
    dynamo_db_repository::DynamoDBRepository, reqwest_google_repository::ReqwestGoogleRepository,
};
use lambda_http::{self, tower::ServiceBuilder, Error};
use presentation::lambda_server::endpoint::fetch_calendar;
use presentation::lambda_server::LambdaServerError;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let db_repository = DynamoDBRepository::new().await;
    let google_repository = ReqwestGoogleRepository::new()
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    let state = AppState::new(google_repository, db_repository);
    let service = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(Arc::new(state)))
        .service_fn(fetch_calendar::<ReqwestGoogleRepository, DynamoDBRepository>);
    lambda_http::run(service).await?;
    Ok(())
}
