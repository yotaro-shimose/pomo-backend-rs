use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct SuccessResponse {}

impl SuccessResponse {
    pub fn new() -> Self {
        Self {}
    }
}
