use actix_web::{web, App, HttpServer};
use reqwest::Client;
use std::env;

mod app_state;
mod handlers;
mod html;
mod models;
mod snowstorm;

use app_state::AppState;
use handlers::{admin, codesystems, fhir_proxy, index, lookup, mapping, search, valuesets};

fn optional_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let snowstorm_base = env::var("SNOWSTORM_BASE").unwrap_or_else(|_| {
        "https://snowstorm-lite-sc-fcap-med-a.apps.ocp1.azure.dso.digital.mod.uk/fhir".to_string()
    });
    let app_admin_password = optional_env("APP_ADMIN_PASSWORD");
    let snowstorm_admin_password = optional_env("SNOWSTORM_ADMIN_PASSWORD");
    let snowstorm_admin_username = optional_env("SNOWSTORM_ADMIN_USERNAME").or_else(|| {
        snowstorm_admin_password
            .as_ref()
            .map(|_| "admin".to_string())
    });

    let state = AppState {
        client: Client::new(),
        snowstorm_base,
        app_admin_password,
        snowstorm_admin_username,
        snowstorm_admin_password,
    };

    println!("Starting SNOMED browser on http://0.0.0.0:8080");
    println!("Using Snowstorm Lite at {}", state.snowstorm_base);
    println!(
        "Protected admin actions: {}",
        if state.app_admin_password.is_some() {
            "enabled"
        } else {
            "disabled"
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(index)
            .service(codesystems)
            .service(valuesets)
            .service(mapping)
            .service(admin)
            .service(search)
            .service(lookup)
            .service(fhir_proxy)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
