use http::Method;
use infrastructure::{
    dynamo_db_repository::DynamoDBRepository, reqwest_google_repository::ReqwestGoogleRepository,
};
use lambda_http::{self, Error};
use presentation::lambda_server::endpoint::push_event;
use runner::run_lambda_runtime;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let handler = push_event::<ReqwestGoogleRepository, DynamoDBRepository>;
    run_lambda_runtime(handler, Method::POST).await
}
