use domain::model::value::AppState;
use http::Method;
use infrastructure::{
    dynamo_db_repository::DynamoDBRepository, reqwest_google_repository::ReqwestGoogleRepository,
};
use lambda_http::{self, tower::ServiceBuilder, Error};
use presentation::lambda_server::endpoint::login;
use presentation::lambda_server::LambdaServerError;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::{Any, CorsLayer};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let db_repository = DynamoDBRepository::new().await;
    let google_repository = ReqwestGoogleRepository::new()
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    let state = AppState::new(google_repository, db_repository);
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::POST])
        .allow_origin(Any);
    let service = ServiceBuilder::new()
        .layer(cors_layer)
        .layer(AddExtensionLayer::new(Arc::new(state)))
        .service_fn(login::<ReqwestGoogleRepository, DynamoDBRepository>);
    lambda_http::run(service).await?;
    Ok(())
}
