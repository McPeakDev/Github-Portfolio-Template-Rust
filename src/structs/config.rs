use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub github_username: String,
    pub linked_in_url: Option<String>,
    pub extended_bio: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}
