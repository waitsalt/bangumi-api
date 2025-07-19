use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BangumiError {
    pub title: String,
    pub description: String,
    pub details: String,
}
