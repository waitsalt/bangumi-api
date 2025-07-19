use serde::{Deserialize, Serialize};

use crate::module::model::Paged;

pub type PagedRevision = Paged<Revision>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Revision {
    pub id: i32,
    pub r#type: i32,
    pub creator: Option<Creator>,
    pub summary: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    pub username: String,
    pub nickname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRevision {
    pub id: i32,
    pub r#type: i32,
    pub creator: Option<Creator>,
    pub summary: String,
    pub created_at: String,
    pub data: Option<std::collections::HashMap<String, PersonRevisionDataItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRevisionDataItem {
    pub prsn_infobox: String,
    pub prsn_summary: String,
    pub profession: PersonRevisionProfession,
    pub extra: RevisionExtra,
    pub prsn_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRevisionProfession {
    pub producer: Option<String>,
    pub mangaka: Option<String>,
    pub artist: Option<String>,
    pub seiyu: Option<String>,
    pub writer: Option<String>,
    pub illustrator: Option<String>,
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionExtra {
    pub img: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRevision {
    pub id: i32,
    pub r#type: i32,
    pub creator: Option<Creator>,
    pub summary: String,
    pub created_at: String,
    pub data: Option<std::collections::HashMap<String, CharacterRevisionDataItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRevisionDataItem {
    pub infobox: String,
    pub summary: String,
    pub name: String,
    pub extra: RevisionExtra,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectRevision {
    pub id: i32,
    pub r#type: i32,
    pub creator: Option<Creator>,
    pub summary: String,
    pub created_at: String,
    pub data: Option<SubjectRevisionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectRevisionData {
    pub field_eps: i32,
    pub field_infobox: String,
    pub field_summary: String,
    pub name: String,
    pub name_cn: String,
    pub platform: i32,
    pub subject_id: i32,
    pub r#type: i32,
    pub type_id: i32,
    pub vote_field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedRevision {
    pub id: i32,
    pub r#type: i32,
    pub creator: Option<Creator>,
    pub summary: String,
    pub created_at: String,
    pub data: Option<serde_json::Value>,
}
