use actix_web::{
    get,
    http::{header::CONTENT_TYPE, StatusCode},
    post, web, HttpResponse, Responder,
};
use std::collections::HashMap;

use crate::app_state::AppState;
use crate::html::{admin_page, browser_page, codesystems_page, mapping_page, valuesets_page};
use crate::models::{ProxyRequest, SearchItem};
use crate::snowstorm::{
    is_destructive_request, lookup_concept, proxy_fhir_request, search_concepts,
};

fn html_response(page: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page)
}

#[get("/")]
pub async fn index() -> impl Responder {
    html_response(browser_page())
}

#[get("/codesystems")]
pub async fn codesystems() -> impl Responder {
    html_response(codesystems_page())
}

#[get("/valuesets")]
pub async fn valuesets() -> impl Responder {
    html_response(valuesets_page())
}

#[get("/mapping")]
pub async fn mapping() -> impl Responder {
    html_response(mapping_page())
}

#[get("/admin")]
pub async fn admin() -> impl Responder {
    html_response(admin_page())
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

#[post("/api/fhir")]
pub async fn fhir_proxy(
    request: web::Json<ProxyRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let request = request.into_inner();
    let destructive = is_destructive_request(&request.method, &request.path);

    if destructive {
        let Some(expected_password) = data.app_admin_password.as_deref() else {
            return HttpResponse::ServiceUnavailable()
                .body("Protected operations are not configured. Set APP_ADMIN_PASSWORD.");
        };

        if request.password.as_deref() != Some(expected_password) {
            return HttpResponse::Unauthorized().body("Password required for protected operation.");
        }
    }

    let admin_auth = if destructive {
        match (
            data.snowstorm_admin_username.as_deref(),
            data.snowstorm_admin_password.as_deref(),
        ) {
            (Some(username), Some(password)) => Some((username, password)),
            _ => {
                return HttpResponse::ServiceUnavailable().body(
                    "Snowstorm admin credentials are not configured. Set SNOWSTORM_ADMIN_USERNAME and SNOWSTORM_ADMIN_PASSWORD.",
                )
            }
        }
    } else {
        None
    };

    match proxy_fhir_request(&data.client, &data.snowstorm_base, &request, admin_auth).await {
        Ok(response) => {
            let status = StatusCode::from_u16(response.status).unwrap_or(StatusCode::BAD_GATEWAY);
            let mut builder = HttpResponse::build(status);
            if let Some(content_type) = response.content_type {
                builder.insert_header((CONTENT_TYPE, content_type));
            }
            builder.body(response.body)
        }
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
