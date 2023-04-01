use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<Data>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Data {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub permission: Vec<Permission>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Permission {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}