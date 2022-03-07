use actix_web::{dev, error, Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IdHeader {
    pub id: String,
}
impl FromRequest for IdHeader {
    type Error = Error;
    type Future = Ready<Result<IdHeader, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let id = req.headers().get("id");
        match id {
            Some(id) => ok(Self {
                id: id.to_str().unwrap().to_string(),
            }),
            None => err(error::ErrorBadRequest("no id specified in the header.")),
        }
    }
}
