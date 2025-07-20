use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: u32,
    pub url: String,
    pub username: String,
    pub nickname: String,
    pub user_group: UserGroup,
    pub avatar: Avatar,
    pub sign: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub url: String,
    pub username: String,
    pub nickname: String,
    pub user_group: UserGroup,
    pub avatar: Avatar,
    pub sign: String,
    pub email: String,
    pub reg_time: String,
    pub time_offset: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Avatar {
    pub large: String,
    pub medium: String,
    pub small: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AvatarType {
    Small,
    Medium,
    Large,
}

/// 用户组
#[repr(u8)]
#[derive(Debug, Serialize_repr, Deserialize_repr)]
pub enum UserGroup {
    Admin = 1,
    BangumiAdmin = 2,
    DoujinAdmin = 3,
    MutedUser = 4,
    BlockedUser = 5,
    WikiAdmin = 9,
    User = 10,
    WikiUser = 11,
}
