use actix_web::Result;
use serde::Deserialize;
use std::fs;

const CLIENT_SECRET_PATH: &str = "./google_auth/client_secret.json";

#[derive(Debug, Deserialize, Clone)]
struct ClientSecretJson {
    web: ClientInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientInfo {
    pub client_id: String,
    pub client_secret: String,
    pub auth_uri: String,
    pub token_uri: String,
}

impl ClientInfo {
    pub fn load() -> Result<ClientInfo> {
        let file = fs::File::open(CLIENT_SECRET_PATH)?;
        let json: ClientSecretJson = serde_json::from_reader(file)?;
        Ok(json.web)
    }
}
