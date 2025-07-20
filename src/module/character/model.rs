use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::{
    model::{BloodType, InfoBox, SimpleImage, Stat},
    subject::model::SubjectType,
};

/// 角色搜索请求结构
///
/// 用于构建角色搜索请求的参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSearch {
    /// 搜索关键词
    pub keyword: String,
    /// 可选的搜索过滤条件
    pub filter: Option<CharacterFilter>,
}

/// 角色搜索过滤条件
///
/// 用于细化角色搜索结果的过滤参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterFilter {
    /// 是否包含NSFW内容
    pub nsfw: Option<bool>,
}

/// 角色详细信息结构
///
/// 表示一个角色的完整信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// 角色ID
    pub id: u32,
    /// 角色名称
    pub name: String,
    /// 角色类型
    pub r#type: CharacterType,
    /// 角色图片信息
    pub images: SimpleImage,
    /// 角色简介
    pub summary: String,
    /// 是否被锁定
    pub locked: bool,
    /// 角色信息框，包含额外属性
    pub infobox: InfoBox,
    /// 角色性别
    pub gender: String,
    /// 角色血型
    pub blood_type: Option<BloodType>,
    /// 角色出生年份
    pub birth_year: Option<u32>,
    /// 角色出生月份
    pub birth_mon: Option<u32>,
    /// 角色出生日期
    pub birth_day: Option<u32>,
    /// 角色统计信息
    pub stat: Stat,
    /// 是否为NSFW内容
    pub nsfw: bool,
}

/// 角色类型枚举
///
/// 表示角色的不同类型，使用u8作为底层存储类型
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CharacterType {
    /// 普通角色
    Character = 1,
    /// 机械角色
    Mechanic = 2,
    /// 舰船角色
    Ship = 3,
    /// 组织角色
    Organization = 4,
}

/// 角色与人物关联结构
///
/// 表示角色与配音演员或创作者的关联信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPerson {
    /// 角色ID
    pub id: u32,
    /// 角色名称
    pub name: String,
    /// 角色类型
    pub r#type: CharacterType,
    /// 角色图片信息
    pub images: SimpleImage,
    /// 关联的条目ID
    pub subject_id: u32,
    /// 关联的条目类型
    pub subject_type: SubjectType,
    /// 关联条目的原名
    pub subject_name: String,
    /// 关联条目的中文名
    pub subject_name_cn: String,
    /// 人物在条目中的职位或角色
    pub staff: String,
}

/// 角色与条目的关联结构
///
/// 表示角色参与的条目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubject {
    /// 条目ID
    pub id: u32,
    /// 条目类型
    pub r#type: SubjectType,
    /// 角色在条目中的职位或角色
    pub staff: String,
    /// 条目原名
    pub name: String,
    /// 条目中文名
    pub name_cn: String,
    /// 条目图片URL
    pub image: String,
}
