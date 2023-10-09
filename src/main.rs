mod api;
mod models;
mod repository;
mod db;
mod app_state;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use env_logger::Env;
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use api::post_api::{create_post};
use repository::user_repo::UserRepo;
use repository::post_repo::PostRepo;
use db::db::DatabaseInstance;
use app_state::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let inst = DatabaseInstance::init().await;
    let user_repo = UserRepo::init(&inst.instance).await;
    let post_repo = PostRepo::init(&inst.instance).await;
    let state = AppState {
        user_repo,
        post_repo
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(create_user)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
            .service(create_post)
    })
        .bind(("127.0.0.1", 6069))?
        .run()
        .await
}
