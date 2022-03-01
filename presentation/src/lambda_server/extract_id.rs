use crate::lambda_server::LambdaServerError;
use domain::model::value::UserId;
use lambda_http::{self, Error, Request};
pub fn extract_id(req: &Request) -> Result<UserId, Error> {
    let id_str = req
        .headers()
        .get("id")
        .ok_or_else(|| LambdaServerError::BadRequest("No ID Specified".to_string()))?
        .to_str()?;
    let user_id = UserId::new(id_str.to_string());
    Ok(user_id)
}
