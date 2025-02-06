use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// Configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking!")
}

// Instantiate and run HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // construct app and configure routes
    let app = move || App::new().configure(general_routes);

    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

/*
The  general_routes  function is used to configure the routes for the application. The  health_check_handler  function is the handler for the  /health  route. The  main  function is the entry point of the application.
To run the application, execute the following command:
cargo run --bin basic-server

The application will start and listen on  http://
*/