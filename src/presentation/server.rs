use super::endpoint::login;
use crate::domain::{
    model::value::AppState,
    repository::{GoogleRepository, UserRepository},
};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};

pub struct Server;
impl Server {
    pub async fn run<G: 'static, U: 'static>(app_state: &'static AppState<G, U>) -> Result<()>
    where
        G: GoogleRepository,
        U: UserRepository,
    {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
        HttpServer::new(|| {
            let data = web::Data::new(app_state.clone());
            let cors = Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST", "PUT"])
                .allow_any_header();
            App::new()
                .app_data(data)
                .wrap(cors)
                .wrap(Logger::default())
                .route("/login", web::to(login::<G, U>))
        })
        .bind("localhost:8000")?
        .run()
        .await?;
        Ok(())
    }
}
