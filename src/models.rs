use serde::Serialize;

#[derive(Serialize)]
pub struct SearchItem {
    pub code: String,
    pub display: String,
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