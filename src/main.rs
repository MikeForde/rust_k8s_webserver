use actix_web::{web, App, HttpServer};
use reqwest::Client;
use std::env;

mod app_state;
mod handlers;
mod html;
mod models;
mod snowstorm;

use app_state::AppState;
use handlers::{index, lookup, search};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let snowstorm_base = env::var("SNOWSTORM_BASE").unwrap_or_else(|_| {
        "https://snowstorm-lite-sc-fcap-med-a.apps.ocp1.azure.dso.digital.mod.uk/fhir".to_string()
    });

    let state = AppState {
        client: Client::new(),
        snowstorm_base,
    };

    println!("Starting SNOMED browser on http://0.0.0.0:8080");
    println!("Using Snowstorm Lite at {}", state.snowstorm_base);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(index)
            .service(search)
            .service(lookup)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}