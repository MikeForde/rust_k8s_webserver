use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;

use crate::app_state::AppState;
use crate::html::INDEX_HTML;
use crate::models::SearchItem;
use crate::snowstorm::{lookup_concept, search_concepts};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX_HTML)
}

#[get("/api/search")]
pub async fn search(
    query: web::Query<HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    let q = query.get("q").cloned().unwrap_or_default();
    if q.trim().is_empty() {
        return HttpResponse::Ok().json(Vec::<SearchItem>::new());
    }

    match search_concepts(&data.client, &data.snowstorm_base, &q).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => {
            if e.starts_with('{') {
                HttpResponse::BadGateway().body(e)
            } else {
                HttpResponse::InternalServerError().body(e)
            }
        }
    }
}

#[get("/api/lookup/{code}")]
pub async fn lookup(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let code = path.into_inner();

    match lookup_concept(&data.client, &data.snowstorm_base, &code).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            if e.starts_with('{') {
                HttpResponse::BadGateway().body(e)
            } else {
                HttpResponse::InternalServerError().body(e)
            }
        }
    }
}