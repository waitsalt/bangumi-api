use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::{
    character::model::CharacterType,
    model::{BloodType, Paged, SimpleImages, Stat},
    subject::model::SubjectType,
};

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PersonType {
    Individual = 1,
    Corporation = 2,
    Association = 3,
}

pub type PagedPerson = Paged<Person>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub r#type: PersonType,
    pub career: Vec<PersonCareer>,
    pub images: Option<SimpleImages>,
    pub short_summary: String,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PersonCareer {
    Producer,
    Mangaka,
    Artist,
    Seiyu,
    Writer,
    Illustrator,
    Actor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedPerson {
    pub id: u32,
    pub name: String,
    pub r#type: PersonType,
    pub career: Vec<PersonCareer>,
    pub images: Option<SimpleImages>,
    pub relation: String,
    pub eps: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPersonsRequest {
    pub keyword: String,
    pub filter: Option<SearchPersonsRequestFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPersonsRequestFilter {
    pub career: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonDetail {
    pub id: i32,
    pub name: String,
    pub r#type: PersonType,
    pub career: Vec<PersonCareer>,
    pub images: Option<SimpleImages>,
    pub summary: String,
    pub locked: bool,
    pub last_modified: String,
    pub infobox: Option<Vec<serde_json::Value>>,
    pub gender: Option<String>,
    pub blood_type: Option<BloodType>,
    pub birth_year: Option<i32>,
    pub birth_mon: Option<i32>,
    pub birth_day: Option<i32>,
    pub stat: Box<Stat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonCharacter {
    pub id: u32,
    pub name: String,
    pub r#type: CharacterType,
    pub images: Option<SimpleImages>,
    pub subject_id: u32,
    pub subject_type: SubjectType,
    pub subject_name: String,
    pub subject_name_cn: String,
    pub staff: Option<String>,
}
