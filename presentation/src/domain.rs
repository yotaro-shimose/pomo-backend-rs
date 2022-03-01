use domain::model::{
    entity::Calendar,
    value::{Code, UserId},
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub authorization_code: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    id: String,
}

impl LoginResponse {
    pub fn new(user_id: UserId) -> Self {
        Self { id: user_id.value }
    }
}

impl From<LoginRequest> for Code {
    fn from(req: LoginRequest) -> Code {
        Code::new(req.authorization_code)
    }
}

#[derive(Debug, Serialize)]
pub struct FrontEndCalendar {
    id: String,
    summary: String,
}

impl From<Calendar> for FrontEndCalendar {
    fn from(calendar: Calendar) -> Self {
        Self {
            id: calendar.id.value,
            summary: calendar.name,
        }
    }
}
