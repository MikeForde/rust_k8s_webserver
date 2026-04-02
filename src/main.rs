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
struct RelatedConcept {
    code: String,
    display: String,
}

#[derive(Serialize)]
struct LookupResponse {
    code: String,
    display: String,
    inactive: Option<bool>,
    effective_time: Option<String>,
    fsn: Option<String>,
    parents: Vec<RelatedConcept>,
    children: Vec<RelatedConcept>,
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
            return HttpResponse::InternalServerError().body(format!("Search request failed: {e}"))
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

async fn fetch_concept_display(
    client: &Client,
    snowstorm_base: &str,
    code: &str,
) -> Result<String, String> {
    let url = format!(
        "{}/CodeSystem/$lookup?system={}&code={}",
        snowstorm_base,
        urlencoding::encode("http://snomed.info/sct"),
        urlencoding::encode(code)
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Lookup request failed for {code}: {e}"))?;

    let body: Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse lookup response for {code}: {e}"))?;

    let display = body
        .get("parameter")
        .and_then(|p| p.as_array())
        .and_then(|params| {
            params.iter().find_map(|p| {
                if p.get("name").and_then(|n| n.as_str()) == Some("display") {
                    p.get("valueString")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                }
            })
        })
        .unwrap_or_else(|| "(display unavailable)".to_string());

    Ok(display)
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
            return HttpResponse::InternalServerError().body(format!("Lookup request failed: {e}"))
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
    let mut inactive = None;
    let mut effective_time = None;
    let mut fsn = None;
    let mut parent_codes: Vec<String> = Vec::new();
    let mut child_codes: Vec<String> = Vec::new();

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
                            inactive = parts.iter().find_map(|part| {
                                if part.get("name").and_then(|n| n.as_str()) == Some("value") {
                                    part.get("valueBoolean").and_then(|v| v.as_bool())
                                } else {
                                    None
                                }
                            });
                        }
                        Some("effectiveTime") => {
                            effective_time = parts.iter().find_map(|part| {
                                if part.get("name").and_then(|n| n.as_str()) == Some("value") {
                                    part.get("valueString")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                } else {
                                    None
                                }
                            });
                        }
                        Some("parent") => {
                            if let Some(parent_code) = parts.iter().find_map(|part| {
                                if part.get("name").and_then(|n| n.as_str()) == Some("value") {
                                    part.get("valueCode")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                } else {
                                    None
                                }
                            }) {
                                parent_codes.push(parent_code);
                            }
                        }
                        Some("child") => {
                            if let Some(child_code) = parts.iter().find_map(|part| {
                                if part.get("name").and_then(|n| n.as_str()) == Some("value") {
                                    part.get("valueCode")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                } else {
                                    None
                                }
                            }) {
                                child_codes.push(child_code);
                            }
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

    let mut parents: Vec<RelatedConcept> = Vec::new();
    for parent_code in parent_codes {
        let parent_display =
            fetch_concept_display(&data.client, &data.snowstorm_base, &parent_code)
                .await
                .unwrap_or_else(|_| "(display unavailable)".to_string());

        parents.push(RelatedConcept {
            code: parent_code,
            display: parent_display,
        });
    }

    let mut children: Vec<RelatedConcept> = Vec::new();
    for child_code in child_codes {
        let child_display = fetch_concept_display(&data.client, &data.snowstorm_base, &child_code)
            .await
            .unwrap_or_else(|_| "(display unavailable)".to_string());

        children.push(RelatedConcept {
            code: child_code,
            display: child_display,
        });
    }

    let result = LookupResponse {
        code,
        display,
        inactive,
        effective_time,
        fsn,
        parents,
        children,
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
    .concept-card {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 12px;
    background: #fafafa;
    margin-bottom: 12px;
  }
  .concept-card.current {
    background: #eef6ff;
    border-color: #bcd3ee;
  }
  .concept-name {
    font-weight: 600;
  }
  .concept-meta {
    color: #666;
    font-size: 12px;
    margin-top: 2px;
  }
  .concept-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 16px;
  }

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
    height: 100vh;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
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
    flex: 1;
    min-height: 0;
  }
  .panel {
    background: white;
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 16px;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
  .panel-body {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
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
  .section-title {
    margin-top: 20px;
    margin-bottom: 8px;
  }
  ul.code-list {
    margin: 0;
    padding-left: 20px;
  }
  ul.code-list li {
    margin-bottom: 6px;
    font-family: monospace;
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
    <div id="results" class="panel-body"></div>
  </div>

  <div class="panel">
    <h2>Concept Details</h2>
    <div id="details" class="panel-body muted">Select a concept from the results.</div>
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
            <h3 class="section-title">Parents</h3>
            ${renderConceptList(data.parents)}

            <h3 class="section-title">Concept</h3>
            <div class="concept-card current">
                <div class="concept-name">${escapeHtml(data.display || '')}</div>
                <div class="concept-meta">${escapeHtml(data.code || '')}</div>
                <div style="margin-top:8px;"><strong>FSN:</strong> ${escapeHtml(data.fsn || '')}</div>
                <div><strong>Inactive:</strong> ${escapeHtml(String(data.inactive))}</div>
                <div><strong>Effective time:</strong> ${escapeHtml(data.effective_time || '')}</div>
            </div>

            <h3 class="section-title">Children</h3>
            ${renderConceptList(data.children)}

            <h3>Raw</h3>
            <pre>${escapeHtml(JSON.stringify(data, null, 2))}</pre>
            `;
      } catch (err) {
        detailsEl.innerHTML = `<div class="muted">Lookup failed: ${escapeHtml(String(err))}</div>`;
      }
    }

    function renderConceptList(items) {
        if (!Array.isArray(items) || items.length === 0) {
            return '<div class="muted">None</div>';
        }

        return `
            <div class="concept-list">
            ${items.map(item => `
                <div class="concept-card">
                <div class="concept-name">${escapeHtml(item.display || '')}</div>
                <div class="concept-meta">(${escapeHtml(item.code || '')})</div>
                </div>
            `).join('')}
            </div>
        `;
        }

    function escapeHtml(str) {
      return String(str)
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
