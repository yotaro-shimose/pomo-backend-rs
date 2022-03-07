use super::endpoint::{
    fetch_calendar, fetch_task, fetch_task_list, fetch_user_config, login, push_event,
    update_user_config,
};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use domain::{
    model::value::AppState,
    repository::{DBRepository, GoogleRepository},
};
pub struct Server;
impl Server {
    pub async fn run<G, U>(app_state: AppState<G, U>) -> Result<()>
    where
        G: GoogleRepository + 'static,
        U: DBRepository + 'static,
    {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
        let factory = move || {
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
                .route("/user", web::put().to(update_user_config::<G, U>))
                .route("/event", web::post().to(push_event::<G, U>))
        };

        HttpServer::new(factory)
            .bind("localhost:8000")?
            .run()
            .await?;
        Ok(())
    }
}
