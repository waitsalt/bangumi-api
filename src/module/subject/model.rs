use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::model::{Images, Paged, Tag};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DailyCalendarItem {
    pub weekday: Option<Weekday>,
    pub items: Option<Vec<LegacySubjectSmall>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Weekday {
    pub en: Option<String>,
    pub cn: Option<String>,
    pub ja: Option<String>,
    pub id: Option<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacySubjectSmall {
    pub id: Option<u32>,
    pub url: Option<String>,
    pub r#type: Option<SubjectType>,
    pub name: Option<String>,
    pub name_cn: Option<String>,
    pub summary: Option<String>,
    pub air_date: Option<String>,
    pub air_weekday: Option<u8>,
    pub images: Option<LegacySubjectSmallImages>,
    pub eps: Option<u32>,
    pub eps_count: Option<u32>,
    pub rating: Option<LegacySubjectSmallRating>,
    pub rank: Option<u32>,
    pub collection: Option<LegacySubjectSmallCollection>,
}

#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SubjectType {
    Book = 1,
    Anime = 2,
    Music = 3,
    Game = 4,
    ThreeDimensional = 6,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacySubjectSmallImages {
    pub large: Option<String>,
    pub common: Option<String>,
    pub medium: Option<String>,
    pub small: Option<String>,
    pub grid: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacySubjectSmallRating {
    pub total: Option<u32>,
    pub count: Option<RatingCount>,
    pub score: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RatingCount {
    #[serde(rename = "1")]
    pub param_1: Option<u32>,
    #[serde(rename = "2")]
    pub param_2: Option<u32>,
    #[serde(rename = "3")]
    pub param_3: Option<u32>,
    #[serde(rename = "4")]
    pub param_4: Option<u32>,
    #[serde(rename = "5")]
    pub param_5: Option<u32>,
    #[serde(rename = "6")]
    pub param_6: Option<u32>,
    #[serde(rename = "7")]
    pub param_7: Option<u32>,
    #[serde(rename = "8")]
    pub param_8: Option<u32>,
    #[serde(rename = "9")]
    pub param_9: Option<u32>,
    #[serde(rename = "10")]
    pub param_10: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacySubjectSmallCollection {
    pub wish: Option<u32>,
    pub collect: Option<u32>,
    pub doing: Option<u32>,
    pub on_hold: Option<u32>,
    pub dropped: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchSubjectsRequest {
    pub keyword: String,
    pub sort: Option<SearchSubjectSort>,
    pub filter: Option<SearchSubjectsRequestFilter>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchSubjectSort {
    Match,
    Heat,
    Rank,
    Score,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchSubjectsRequestFilter {
    pub r#type: Option<Vec<SubjectType>>,
    pub meta_tags: Option<Vec<String>>,
    pub tag: Option<Vec<String>>,
    pub air_date: Option<Vec<String>>,
    pub rating: Option<Vec<String>>,
    pub rank: Option<Vec<String>>,
    pub nsfw: Option<bool>,
}

pub type PagedSubject = Paged<Subject>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
    pub id: u32,
    pub r#type: SubjectType,
    pub name: String,
    pub name_cn: String,
    pub summary: String,
    pub series: bool,
    pub nsfw: bool,
    pub locked: bool,
    pub date: Option<String>,
    pub platform: String,
    pub images: Images,
    pub infobox: Option<Infobox>,
    pub volumes: u32,
    pub eps: u32,
    pub total_episodes: Option<u32>, // 文档中要求为必须有 实际测试中有时候没有
    pub rating: Rating,
    pub collection: Collection,
    pub meta_tags: Vec<String>,
    pub tags: Vec<Tag>,
}

pub type Infobox = Vec<InfoboxItem>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InfoboxItem {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rating {
    pub rank: u32,
    pub total: u32,
    pub count: RatingCount,
    pub score: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collection {
    pub wish: u32,
    pub collect: u32,
    pub doing: u32,
    pub on_hold: u32,
    pub dropped: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubjectCategory {
    Book(SubjectBookCategory),
    Anime(SubjectAnimeCategory),
    Game(SubjectGameCategory),
    Real(SubjectRealCategory),
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SubjectBookCategory {
    Other = 0,
    Comic = 1001,
    Novel = 1002,
    Artbook = 1003,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SubjectAnimeCategory {
    Other = 0,
    Tv = 1,
    Ova = 2,
    Movie = 3,
    Web = 5,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SubjectGameCategory {
    Other = 0,
    Game = 4001,
    Software = 4002,
    Expansion = 4003,
    BoardGame = 4005,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SubjectRealCategory {
    Other = 0,
    JapaneseDrama = 1,
    EuroAmericanDrama = 2,
    ChineseDrama = 3,
    TvDrama = 6001,
    Movie = 6002,
    Performance = 6003,
    VarietyShow = 6004,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubjectSort {
    Date,
    Rank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedSubject {
    pub id: u32,
    pub r#type: SubjectType,
    pub staff: String,
    pub name: String,
    pub name_cn: String,
    pub images: Option<Images>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectRelation {
    pub id: u32,
    pub r#type: SubjectType,
    pub name: String,
    pub name_cn: String,
    pub images: Option<Images>,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimSubject {
    pub id: i32,
    pub r#type: SubjectType,
    pub name: String,
    pub name_cn: String,
    pub short_summary: String,
    pub date: Option<String>,
    pub images: Images,
    pub volumes: i32,
    pub eps: i32,
    pub collection_total: i32,
    pub score: f64,
    pub rank: i32,
    pub tags: Vec<Tag>,
}
