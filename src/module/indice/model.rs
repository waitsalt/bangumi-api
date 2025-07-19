use serde::{Deserialize, Serialize};

use crate::module::{model::Stat, revision::model::Creator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub total: Option<i32>,
    pub stat: Stat,
    pub created_at: String,
    pub updated_at: String,
    pub creator: Creator,
    pub ban: bool,
    pub nsfw: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexBasicInfo {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSubjectAddInfo {
    pub subject_id: Option<i32>,
    pub sort: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSubjectEditInfo {
    pub sort: Option<i32>,
    pub comment: Option<String>,
}
