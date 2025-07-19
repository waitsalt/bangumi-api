use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::{
    character::model::CharacterType,
    episode::model::Episode,
    model::{Paged, SimpleImages},
    person::model::{PersonCareer, PersonType},
    user::model::UserSubjectCollection,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub wish: i32,
    pub collect: i32,
    pub doing: i32,
    pub on_hold: i32,
    pub dropped: i32,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CollectionType {
    Wish = 1,
    Done = 2,
    Doing = 3,
    OnHold = 4,
    Dropped = 5,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EpisodeCollectionType {
    Not = 0,
    Wish = 1,
    Done = 2,
    Dropped = 3,
}

pub type PagedUserCollection = Paged<UserSubjectCollection>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubjectCollectionModifyPayload {
    pub r#type: Option<CollectionType>,
    pub rate: Option<i32>,
    pub ep_status: Option<i32>,
    pub vol_status: Option<i32>,
    pub comment: Option<String>,
    pub private: Option<bool>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEpisodeCollection {
    pub episode: Episode,
    pub r#type: CollectionType,
    pub updated_at: i32,
}

pub type PagedUserCharacterCollection = Paged<UserCharacterCollection>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCharacterCollection {
    pub id: i32,
    pub name: String,
    pub r#type: CharacterType,
    pub images: Option<SimpleImages>,
    pub created_at: String,
}

pub type PagedUserPersonCollection = Paged<UserPersonCollection>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPersonCollection {
    pub id: i32,
    pub name: String,
    pub r#type: PersonType,
    pub career: Vec<PersonCareer>,
    pub images: Option<SimpleImages>,
    pub created_at: String,
}
