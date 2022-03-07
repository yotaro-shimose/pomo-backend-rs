use http::Method;
use infrastructure::{
    dynamo_db_repository::DynamoDBRepository, reqwest_google_repository::ReqwestGoogleRepository,
};
use lambda_http::{self, tower::ServiceBuilder, Error};
use presentation::lambda_server::endpoint::login;
use runner::{init_logger, load_lambda_app_state};
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::{Any, CorsLayer};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    init_logger();
    let state = load_lambda_app_state().await?;
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
