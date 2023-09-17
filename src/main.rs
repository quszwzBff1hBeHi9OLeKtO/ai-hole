use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use std::env;

mod action;
mod config;
mod error;
mod processer;
mod role;
mod service;
mod state;
mod stats;

#[actix_rt::main]
async fn main() {
    // Misc
    tracing_subscriber::fmt::init();

    // Config
    let port: u16 = env::var("PORT")
        .map(|port| {
            port.parse()
                .expect("PORT environment variable could not be converted to a u16")
        })
        .unwrap_or(8080);

    // State
    let http_client = web::Data::new(reqwest::Client::new());
    let app_state = web::Data::new(state::AppState::new());

    tracing::info!("Listening on http://[::1]:{port}");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(http_client.clone())
            .app_data(app_state.clone())
            .service(web::scope("/proxy").configure(service::proxy::service))
            .service(web::scope("/metrics").configure(service::prometheus::service))
            .service(web::scope("").configure(service::info::service))
    })
    .bind(("::", port)) // Binds on ipv4 and ipv6 for some reason
    .unwrap() // Let the panic guide the user to fix their shit
    .run()
    .await
    .expect("failed to start server");
}
