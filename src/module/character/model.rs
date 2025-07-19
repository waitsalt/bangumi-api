use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::{
    model::{BloodType, Paged, SimpleImages, Stat},
    person::model::Person,
    subject::model::SubjectType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCharactersRequest {
    pub keyword: String,
    pub filter: Option<SearchCharactersRequestFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCharactersRequestFilter {
    pub nsfw: Option<bool>,
}
pub type PagedCharacter = Paged<Character>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub r#type: CharacterType,
    pub images: Option<SimpleImages>,
    pub summary: String,
    pub locked: bool,
    pub infobox: Option<Vec<serde_json::Value>>,
    pub gender: Option<String>,
    pub blood_type: Option<BloodType>,
    pub birth_year: Option<i32>,
    pub birth_mon: Option<i32>,
    pub birth_day: Option<i32>,
    pub stat: Stat,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CharacterType {
    Character = 1,
    Mechanic = 2,
    Ship = 3,
    Organization = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedCharacter {
    pub id: u32,
    pub name: String,
    pub r#type: CharacterType,
    pub images: Option<SimpleImages>,
    pub relation: String,
    pub actors: Option<Vec<Person>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPerson {
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
