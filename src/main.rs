use actix_web::Result;
use backend::presentation::Server;
#[actix_web::main]
async fn main() -> Result<()> {
    Server::run();
    Ok(())
}
