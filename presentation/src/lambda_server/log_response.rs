use lambda_http::{Error, Response};
use log::{error, info};

pub fn log_response(ret: &Result<Response<String>, Error>) {
    match ret {
        Ok(response) => {
            info!("Response: {}", response.body())
        }
        Err(err) => {
            error!("Error: {}", err)
        }
    }
}
