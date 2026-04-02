use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::env;

#[derive(Clone)]
struct AppState {
    client: Client,
    snowstorm_base: String,
}

#[derive(Serialize)]
struct SearchItem {
    code: String,
    display: String,
}

#[derive(Serialize)]
struct LookupResponse {
    code: String,
    display: String,
    version: Option<String>,
    inactive: Option<bool>,
    module_id: Option<String>,
    effective_time: Option<String>,
    fsn: Option<String>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX_HTML)
}

#[get("/api/search")]
async fn search(
    query: web::Query<std::collections::HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    let q = query.get("q").cloned().unwrap_or_default();
    if q.trim().is_empty() {
        return HttpResponse::Ok().json(Vec::<SearchItem>::new());
    }

    let url = format!(
        "{}/ValueSet/$expand?url={}&filter={}",
        data.snowstorm_base,
        urlencoding::encode("http://snomed.info/sct?fhir_vs"),
        urlencoding::encode(&q)
    );

    let resp = match data.client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Search request failed: {e}"))
        }
    };

    let status = resp.status();
    let body: Value = match resp.json().await {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to parse search response: {e}"))
        }
    };

    if !status.is_success() {
        return HttpResponse::BadGateway().json(body);
    }

    let results = body
        .get("expansion")
        .and_then(|e| e.get("contains"))
        .and_then(|c| c.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    Some(SearchItem {
                        code: item.get("code")?.as_str()?.to_string(),
                        display: item.get("display")?.as_str()?.to_string(),
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    HttpResponse::Ok().json(results)
}

#[get("/api/lookup/{code}")]
async fn lookup(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let code = path.into_inner();

    let url = format!(
        "{}/CodeSystem/$lookup?system={}&code={}",
        data.snowstorm_base,
        urlencoding::encode("http://snomed.info/sct"),
        urlencoding::encode(&code)
    );

    let resp = match data.client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Lookup request failed: {e}"))
        }
    };

    let status = resp.status();
    let body: Value = match resp.json().await {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to parse lookup response: {e}"))
        }
    };

    if !status.is_success() {
        return HttpResponse::BadGateway().json(body);
    }

    let parameters = body
        .get("parameter")
        .and_then(|p| p.as_array())
        .cloned()
        .unwrap_or_default();

    let mut display = String::new();
    let mut version = None;
    let mut inactive = None;
    let mut module_id = None;
    let mut effective_time = None;
    let mut fsn = None;

    for p in &parameters {
        let name = p.get("name").and_then(|n| n.as_str()).unwrap_or("");

        match name {
            "display" => {
                display = p
                    .get("valueString")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
            }
            "version" => {
                version = p
                    .get("valueString")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
            "property" => {
                if let Some(parts) = p.get("part").and_then(|v| v.as_array()) {
                    let prop_code = parts.iter().find_map(|part| {
                        if part.get("name").and_then(|n| n.as_str()) == Some("code") {
                            part.get("valueCode").and_then(|v| v.as_str())
                        } else {
                            None
                        }
                    });

                    match prop_code {
                        Some("inactive") => {
                            inactive = parts.iter().find_map(|part| part.get("valueBoolean").and_then(|v| v.as_bool()));
                        }
                        Some("moduleId") => {
                            module_id = parts.iter().find_map(|part| {
                                part.get("valueCode")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string())
                            });
                        }
                        Some("effectiveTime") => {
                            effective_time = parts.iter().find_map(|part| {
                                part.get("valueString")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string())
                            });
                        }
                        _ => {}
                    }
                }
            }
            "designation" => {
                if let Some(parts) = p.get("part").and_then(|v| v.as_array()) {
                    let use_code = parts.iter().find_map(|part| {
                        if part.get("name").and_then(|n| n.as_str()) == Some("use") {
                            part.get("valueCoding")
                                .and_then(|vc| vc.get("code"))
                                .and_then(|c| c.as_str())
                        } else {
                            None
                        }
                    });

                    if use_code == Some("900000000000003001") {
                        fsn = parts.iter().find_map(|part| {
                            if part.get("name").and_then(|n| n.as_str()) == Some("value") {
                                part.get("valueString")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string())
                            } else {
                                None
                            }
                        });
                    }
                }
            }
            _ => {}
        }
    }

    let result = LookupResponse {
        code,
        display,
        version,
        inactive,
        module_id,
        effective_time,
        fsn,
    };

    HttpResponse::Ok().json(result)
}

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

const INDEX_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>Simple SNOMED Browser</title>
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <style>
    body {
      font-family: Arial, sans-serif;
      margin: 0;
      background: #f6f8fa;
      color: #222;
    }
    .wrap {
      max-width: 960px;
      margin: 0 auto;
      padding: 24px;
    }
    h1 {
      margin-top: 0;
    }
    .search-row {
      display: flex;
      gap: 12px;
      margin-bottom: 16px;
    }
    input {
      flex: 1;
      padding: 12px;
      font-size: 16px;
    }
    button {
      padding: 12px 16px;
      font-size: 16px;
      cursor: pointer;
    }
    .grid {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 16px;
    }
    .panel {
      background: white;
      border: 1px solid #ddd;
      border-radius: 8px;
      padding: 16px;
      min-height: 320px;
    }
    .result {
      padding: 10px;
      border-bottom: 1px solid #eee;
      cursor: pointer;
    }
    .result:hover {
      background: #f1f5f9;
    }
    .code {
      color: #555;
      font-size: 13px;
    }
    .display {
      font-weight: 600;
    }
    pre {
      white-space: pre-wrap;
      word-break: break-word;
      background: #f8f8f8;
      padding: 12px;
      border-radius: 6px;
      overflow: auto;
    }
    .muted {
      color: #666;
    }
  </style>
</head>
<body>
  <div class="wrap">
    <h1>Simple SNOMED Browser</h1>
    <p class="muted">Search SNOMED CT terms using Snowstorm Lite.</p>

    <div class="search-row">
      <input id="searchBox" placeholder="Type a term, e.g. appendicitis, asthma, penicillin" />
      <button id="searchBtn">Search</button>
    </div>

    <div class="grid">
      <div class="panel">
        <h2>Results</h2>
        <div id="results"></div>
      </div>

      <div class="panel">
        <h2>Concept Details</h2>
        <div id="details" class="muted">Select a concept from the results.</div>
      </div>
    </div>
  </div>

  <script>
    const searchBox = document.getElementById('searchBox');
    const searchBtn = document.getElementById('searchBtn');
    const resultsEl = document.getElementById('results');
    const detailsEl = document.getElementById('details');

    async function runSearch() {
      const q = searchBox.value.trim();
      if (!q) {
        resultsEl.innerHTML = '<div class="muted">Enter a search term.</div>';
        return;
      }

      resultsEl.innerHTML = '<div class="muted">Searching...</div>';
      detailsEl.innerHTML = '<div class="muted">Select a concept from the results.</div>';

      try {
        const resp = await fetch(`/api/search?q=${encodeURIComponent(q)}`);
        const data = await resp.json();

        if (!Array.isArray(data) || data.length === 0) {
          resultsEl.innerHTML = '<div class="muted">No results found.</div>';
          return;
        }

        resultsEl.innerHTML = '';
        data.forEach(item => {
          const row = document.createElement('div');
          row.className = 'result';
          row.innerHTML = `
            <div class="display">${escapeHtml(item.display)}</div>
            <div class="code">${escapeHtml(item.code)}</div>
          `;
          row.addEventListener('click', () => loadDetails(item.code));
          resultsEl.appendChild(row);
        });
      } catch (err) {
        resultsEl.innerHTML = `<div class="muted">Search failed: ${escapeHtml(String(err))}</div>`;
      }
    }

    async function loadDetails(code) {
      detailsEl.innerHTML = '<div class="muted">Loading details...</div>';

      try {
        const resp = await fetch(`/api/lookup/${encodeURIComponent(code)}`);
        const data = await resp.json();

        detailsEl.innerHTML = `
          <p><strong>Display:</strong> ${escapeHtml(data.display || '')}</p>
          <p><strong>Code:</strong> ${escapeHtml(data.code || '')}</p>
          <p><strong>FSN:</strong> ${escapeHtml(data.fsn || '')}</p>
          <p><strong>Version:</strong> ${escapeHtml(data.version || '')}</p>
          <p><strong>Inactive:</strong> ${escapeHtml(String(data.inactive))}</p>
          <p><strong>Module ID:</strong> ${escapeHtml(data.module_id || '')}</p>
          <p><strong>Effective time:</strong> ${escapeHtml(data.effective_time || '')}</p>
          <h3>Raw</h3>
          <pre>${escapeHtml(JSON.stringify(data, null, 2))}</pre>
        `;
      } catch (err) {
        detailsEl.innerHTML = `<div class="muted">Lookup failed: ${escapeHtml(String(err))}</div>`;
      }
    }

    function escapeHtml(str) {
      return str
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;')
        .replaceAll('"', '&quot;')
        .replaceAll("'", '&#39;');
    }

    searchBtn.addEventListener('click', runSearch);
    searchBox.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') runSearch();
    });
  </script>
</body>
</html>
"#;