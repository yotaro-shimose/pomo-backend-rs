use crate::lambda_server::LambdaServerError;
use lambda_http::{self, Body, Error};
use serde::de::DeserializeOwned;
pub fn parse_body<T: DeserializeOwned>(body: &Body) -> Result<T, Error> {
    if let Body::Text(body) = body {
        serde_json::from_str::<T>(body).map_err(|_| {
            LambdaServerError::BadRequest(format!("Could not parse request body {}", body)).into()
        })
    } else {
        Err(LambdaServerError::BadRequest("Request is empty or bytes".to_string()).into())
    }
}
