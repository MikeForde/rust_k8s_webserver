use reqwest::Client;
use serde_json::Value;

use crate::models::{LookupResponse, RelatedConcept, SearchItem};

pub async fn search_concepts(
    client: &Client,
    snowstorm_base: &str,
    q: &str,
) -> Result<Vec<SearchItem>, String> {
    let url = format!(
        "{}/ValueSet/$expand?url={}&filter={}",
        snowstorm_base,
        urlencoding::encode("http://snomed.info/sct?fhir_vs"),
        urlencoding::encode(q)
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Search request failed: {e}"))?;

    let status = resp.status();

    let body: Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse search response: {e}"))?;

    if !status.is_success() {
        return Err(body.to_string());
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

    Ok(results)
}

pub async fn fetch_concept_display(
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

pub async fn lookup_concept(
    client: &Client,
    snowstorm_base: &str,
    code: &str,
) -> Result<LookupResponse, String> {
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
        .map_err(|e| format!("Lookup request failed: {e}"))?;

    let status = resp.status();

    let body: Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse lookup response: {e}"))?;

    if !status.is_success() {
        return Err(body.to_string());
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
        let parent_display = fetch_concept_display(client, snowstorm_base, &parent_code)
            .await
            .unwrap_or_else(|_| "(display unavailable)".to_string());

        parents.push(RelatedConcept {
            code: parent_code,
            display: parent_display,
        });
    }

    let mut children: Vec<RelatedConcept> = Vec::new();
    for child_code in child_codes {
        let child_display = fetch_concept_display(client, snowstorm_base, &child_code)
            .await
            .unwrap_or_else(|_| "(display unavailable)".to_string());

        children.push(RelatedConcept {
            code: child_code,
            display: child_display,
        });
    }

    Ok(LookupResponse {
        code: code.to_string(),
        display,
        inactive,
        effective_time,
        fsn,
        parents,
        children,
    })
}