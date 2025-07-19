use serde::{Deserialize, Serialize};

use crate::module::{
    collection::model::CollectionType,
    subject::model::{SlimSubject, SubjectType},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: u32,
    pub username: String,
    pub nickname: String,
    pub user_group: UserGroup,
    pub avatar: Avatar,
    pub sign: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub nickname: String,
    pub user_group: UserGroup,
    pub avatar: Avatar,
    pub sign: String,
    pub email: String,
    pub reg_time: String,
    pub time_offset: Option<u32>,
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
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubjectCollection {
    pub subject_id: i32,
    pub subject_type: SubjectType,
    pub rate: i32,
    pub r#type: CollectionType,
    pub comment: Option<String>,
    pub tags: Vec<String>,
    pub ep_status: i32,
    pub vol_status: i32,
    /// 本时间并不代表条目的收藏时间。修改评分，评价，章节观看状态等收藏信息时未更新此时间是一个 bug。请不要依赖此特性
    pub updated_at: String,
    pub private: bool,
    pub subject: Option<SlimSubject>,
}
