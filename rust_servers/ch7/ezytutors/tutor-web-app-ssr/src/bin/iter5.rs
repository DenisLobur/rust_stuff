use routes::app_config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use sqlx::PgPool;
use tera::Tera;
use crate::state::AppState;

#[path = "../iter5/mod.rs"]
mod dbaccess;
#[path = "../iter5/errors.rs"]
mod errors;
#[path = "../iter5/mod.rs"]
mod handlers;
#[path = "../iter5/mod.rs"]
mod models;
#[path = "../iter5/routes.rs"]
mod routes;
#[path = "../iter5/state.rs"]
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //Start HTTP server
    let host_port = env::var("HOST_PORT").expect(
        "HOST:PORT address is not set in .env file");
    println!("Listening on: {}", &host_port);
    let database_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // Construct App State
    let shared_data = web::Data::new(AppState { db: db_pool });
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"),
        "/static/iter5/**/*")).unwrap();
        App::new()
            .data(tera)
            .app_data(shared_data.clone())
            .configure(app_config)
    })
        .bind(&host_port)?
        .run()
        .await
}