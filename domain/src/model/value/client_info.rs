use actix_web::{error, Result};
use envy;
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
    pub fn load() -> Result<Self> {
        if let Ok(info) = ClientInfo::load_from_env() {
            Ok(info)
        } else {
            ClientInfo::load_from_file()
        }
    }

    fn load_from_env() -> Result<Self> {
        envy::from_env().map_err(error::ErrorInternalServerError)
    }

    fn load_from_file() -> Result<Self> {
        let file = fs::File::open(CLIENT_SECRET_PATH)?;
        let json: ClientSecretJson = serde_json::from_reader(file)?;
        Ok(json.web)
    }
}
