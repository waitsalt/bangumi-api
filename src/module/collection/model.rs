use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::{
    character::model::CharacterType,
    episode::model::Episode,
    model::{Image, SimpleImage, Tag},
    person::model::{PersonCareer, PersonType},
    subject::model::SubjectType,
};

/// 收藏状态统计信息
///
/// 包含不同收藏状态的数量统计，用于展示条目或内容的收藏分布情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    /// "想看"状态的数量
    pub wish: Option<u32>,
    /// "已收藏"状态的数量
    pub collect: Option<u32>,
    /// "在看"状态的数量
    pub doing: Option<u32>,
    /// "搁置"状态的数量
    pub on_hold: Option<u32>,
    /// "抛弃"状态的数量
    pub dropped: Option<u32>,
}

/// 收藏类型枚举
///
/// 表示用户对条目（如动画、书籍等）的收藏状态分类，使用u8作为底层存储类型
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CollectionType {
    /// 想看（计划观看/阅读）
    Wish = 1,
    /// 已看（已完成）
    Done = 2,
    /// 在看（进行中）
    Doing = 3,
    /// 搁置（暂停中）
    OnHold = 4,
    /// 抛弃（已放弃）
    Dropped = 5,
}

/// 单集收藏状态枚举
///
/// 表示用户对条目中单个剧集的收藏/观看状态，使用u8作为底层存储类型
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CollectionEpisodeType {
    /// 未处理（未标记状态）
    Not = 0,
    /// 想看（计划观看该集）
    Wish = 1,
    /// 已看（已完成观看该集）
    Done = 2,
    /// 抛弃（放弃观看该集）
    Dropped = 3,
}

/// 更新条目收藏状态的请求参数
///
/// 用于向API提交更新用户对某个条目的收藏信息，包含状态、评分、进度等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSubjectUpdate {
    /// 收藏类型（可选，更新为指定状态）
    pub r#type: Option<CollectionType>,
    /// 评分（可选，1-10分）
    pub rate: Option<u32>,
    /// 剧集进度（可选，已观看的集数）
    pub ep_status: Option<u32>,
    /// 卷进度（可选，已阅读的卷数）
    pub vol_status: Option<u32>,
    /// 评论（可选，用户对条目的评论）
    pub comment: Option<String>,
    /// 是否私密（可选，设置收藏信息是否私密）
    pub private: Option<bool>,
    /// 标签（可选，用户为条目添加的标签列表）
    pub tags: Option<Vec<String>>,
}

/// 单集收藏详情
///
/// 包含单个剧集的信息及其对应的收藏状态和更新时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEpisode {
    /// 剧集基本信息
    pub episode: Episode,
    /// 该集的收藏状态
    pub r#type: CollectionEpisodeType,
    /// 状态更新时间（时间戳）
    pub updated_at: u32,
}

/// 用户收藏的角色信息
///
/// 表示用户收藏的角色详情，包含角色ID、名称、类型等基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionCharacter {
    /// 角色ID
    pub id: u32,
    /// 角色名称
    pub name: String,
    /// 角色类型
    pub r#type: CharacterType,
    /// 角色图片信息（可选）
    pub images: Option<SimpleImage>,
    /// 收藏时间（时间戳字符串）
    pub created_at: String,
}

/// 用户收藏的人物信息
///
/// 表示用户收藏的人物详情，包含人物ID、名称、职业等基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionPerson {
    /// 人物ID
    pub id: u32,
    /// 人物名称
    pub name: String,
    /// 人物类型
    pub r#type: PersonType,
    /// 人物职业（可选，如配音演员、导演等）
    pub career: Option<Vec<PersonCareer>>,
    /// 人物图片信息（可选）
    pub images: Option<SimpleImage>,
    /// 收藏时间（时间戳字符串）
    pub created_at: String,
}

/// 用户对条目的收藏详情
///
/// 包含用户对某个条目的收藏状态、评分、进度等信息，以及关联的条目基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSubject {
    /// 条目ID
    pub subject_id: u32,
    /// 条目类型
    pub subject_type: SubjectType,
    /// 评分（1-10分）
    pub rate: u32,
    /// 收藏类型
    pub r#type: CollectionType,
    /// 评论内容（可选）
    pub comment: Option<String>,
    /// 标签列表
    pub tags: Vec<String>,
    /// 剧集进度（已观看集数）
    pub ep_status: u32,
    /// 卷进度（已阅读卷数）
    pub vol_status: u32,
    /// 最后更新时间（时间戳字符串）
    pub updated_at: String,
    /// 是否私密（true表示仅自己可见）
    pub private: bool,
    /// 关联的条目详情（可选）
    pub subject: Option<UserSubject>,
}

/// 收藏条目的详细信息
///
/// 包含被收藏条目的基本信息，如名称、评分、标签等，用于展示收藏条目详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubject {
    /// 条目ID
    pub id: u32,
    /// 条目类型
    pub r#type: SubjectType,
    /// 条目原名
    pub name: String,
    /// 条目中文名
    pub name_cn: String,
    /// 简短简介
    pub short_summary: String,
    /// 发布日期
    pub date: String,
    /// 图片信息
    pub images: Image,
    /// 总卷数
    pub volumes: u32,
    /// 总集数
    pub eps: u32,
    /// 总收藏数
    pub collection_total: u32,
    /// 综合评分
    pub score: f64,
    /// 排名
    pub rank: u32,
    /// 标签列表
    pub tags: Vec<Tag>,
}

/// 批量更新单集收藏状态的请求参数
///
/// 用于一次性更新多个剧集的收藏状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEpisodesUpdate {
    /// 剧集ID列表（需更新状态的剧集）
    pub episode_id: Vec<u32>,
    /// 目标收藏状态（更新后的状态）
    pub r#type: CollectionEpisodeType,
}

/// 单个更新单集收藏状态的请求参数
///
/// 用于更新单个剧集的收藏状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionEpisodeUpdate {
    /// 目标收藏状态（更新后的状态）
    pub r#type: CollectionEpisodeType,
}
