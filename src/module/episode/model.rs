use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::model::Paged;

pub type PagedEpisode = Paged<Episode>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: i32,
    pub r#type: EpisodeType,
    pub name: String,
    pub name_cn: String,
    pub sort: f64,
    pub ep: Option<f64>,
    pub airdate: String,
    pub comment: i32,
    pub duration: String,
    pub desc: String,
    pub disc: i32,
    pub duration_seconds: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EpisodeType {
    Normal = 0,
    SP = 1,
    OP = 2,
    ED = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeDetail {
    pub id: i32,
    pub r#type: EpisodeType,
    pub name: String,
    pub name_cn: String,
    pub sort: f64,
    pub ep: Option<f64>,
    pub airdate: String,
    pub comment: i32,
    pub duration: String,
    pub desc: String,
    pub disc: i32,
    pub subject_id: i32,
}
