use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct SearchItem {
    pub code: String,
    pub display: String,
    pub inactive: Option<bool>,
}

#[derive(Serialize)]
pub struct RelatedConcept {
    pub code: String,
    pub display: String,
}

#[derive(Serialize)]
pub struct LookupResponse {
    pub code: String,
    pub display: String,
    pub inactive: Option<bool>,
    pub effective_time: Option<String>,
    pub fsn: Option<String>,
    pub parents: Vec<RelatedConcept>,
    pub children: Vec<RelatedConcept>,
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct ProxyRequest {
    pub method: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub query: Vec<QueryParam>,
    pub body: Option<Value>,
    pub password: Option<String>,
}
