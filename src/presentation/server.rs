use super::endpoint::{fetch_calendar, fetch_task, fetch_task_list, fetch_user_config, login};
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
            let cors = Cors::permissive();
            App::new()
                .app_data(data)
                .wrap(Logger::default())
                .wrap(cors)
                .route("/login", web::post().to(login::<G, U>))
                .route("/task", web::get().to(fetch_task::<G, U>))
                .route("/userConfig", web::get().to(fetch_user_config::<G, U>))
                .route("/taskList", web::get().to(fetch_task_list::<G, U>))
                .route("/calendar", web::get().to(fetch_calendar::<G, U>))
        })
        .bind("localhost:8000")?
        .run()
        .await?;
        Ok(())
    }
}
