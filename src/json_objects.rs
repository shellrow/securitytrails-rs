use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Subdomains {
    pub endpoint: String,
    pub subdomains: Vec<String>,
}
