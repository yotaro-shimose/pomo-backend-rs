use super::endpoint::{fetch_task, fetch_user_config, login};
use crate::domain::{
    model::value::AppState,
    repository::{DBRepository, GoogleRepository},
};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};

pub struct Server;
impl Server {
    pub async fn run<G, U>(app_state: AppState<G, U>) -> Result<()>
    where
        G: GoogleRepository + 'static,
        U: DBRepository + 'static,
    {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
        HttpServer::new(move || {
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
                .route("/task", web::to(fetch_task::<G, U>))
                .route("/userConfig", web::to(fetch_user_config::<G, U>))
        })
        .bind("localhost:8000")?
        .run()
        .await?;
        Ok(())
    }
}
