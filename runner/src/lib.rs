use domain::model::value::AppState;
use http::Method;
use infrastructure::{
    dynamo_db_repository::DynamoDBRepository, reqwest_google_repository::ReqwestGoogleRepository,
};
use std::future::Future;

use lambda_http::Request;
use lambda_http::{self, tower::ServiceBuilder, Error, Response};
use presentation::lambda_server::LambdaServerError;
use std::sync::Arc;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::{Any, CorsLayer};

async fn load_lambda_app_state(
) -> Result<AppState<ReqwestGoogleRepository, DynamoDBRepository>, LambdaServerError> {
    let db_repository = DynamoDBRepository::new().await;
    let google_repository = ReqwestGoogleRepository::new()
        .map_err(|err| LambdaServerError::InternalServerError(err.to_string()))?;
    Ok(AppState::new(google_repository, db_repository))
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

pub async fn run_lambda_runtime<F, Fut>(f: F, method: Method) -> Result<(), Error>
where
    F: Fn(Request) -> Fut + Send,
    Fut: Future<Output = Result<Response<String>, Error>> + Send,
{
    init_logger();
    let state = load_lambda_app_state().await?;
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(vec![method])
        .allow_origin(Any);
    let service = ServiceBuilder::new()
        .layer(cors_layer)
        .layer(AddExtensionLayer::new(Arc::new(state)))
        .service_fn(f);
    lambda_http::run(service).await?;
    Ok(())
}
