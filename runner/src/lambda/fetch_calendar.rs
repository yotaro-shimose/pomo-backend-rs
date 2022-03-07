use http::Method;
use infrastructure::{
    dynamo_db_repository::DynamoDBRepository, reqwest_google_repository::ReqwestGoogleRepository,
};
use lambda_http::{self, Error};
use presentation::lambda_server::endpoint::fetch_calendar;
use runner::run_lambda_runtime;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let handler = fetch_calendar::<ReqwestGoogleRepository, DynamoDBRepository>;
    run_lambda_runtime(handler, Method::GET).await
}
