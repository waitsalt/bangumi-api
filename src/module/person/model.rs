use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::{
    character::model::CharacterType,
    model::{BloodType, Image, SimpleImage, Stat},
    subject::model::SubjectType,
};

/// 人物类型枚举
///
/// 用于区分不同类型的人物主体，底层使用u8存储以优化空间
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PersonType {
    /// 个人（自然人）
    Individual = 1,
    /// 企业（法人实体）
    Corporation = 2,
    /// 团体（组织或协会）
    Association = 3,
}

/// 人物基本信息结构体
///
/// 包含人物的核心标识信息、职业和基础展示数据，用于列表展示等场景
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// 人物唯一标识符
    pub id: u32,
    /// 人物姓名
    pub name: String,
    /// 人物类型（个人、企业或团体）
    pub r#type: PersonType,
    /// 人物职业列表（如制作人、声优等）
    pub career: Vec<PersonCareer>,
    /// 人物图片信息
    pub images: SimpleImage,
    /// 简短简介
    pub short_summary: String,
    /// 是否被锁定（内容不可编辑）
    pub locked: bool,
}

/// 人物职业枚举
///
/// 定义人物可能从事的职业类型，序列化时使用小写字母
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PersonCareer {
    /// 制作人
    Producer,
    /// 漫画家
    Mangaka,
    /// 艺术家/音乐人
    Artist,
    /// 声优（配音演员）
    Seiyu,
    /// 编剧
    Writer,
    /// 插画师
    Illustrator,
    /// 演员
    Actor,
}

/// 人物搜索请求结构
///
/// 用于构建人物搜索的查询参数，支持关键词搜索和职业筛选
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonSearch {
    /// 搜索关键词（人物姓名或相关关键词）
    pub keyword: String,
    /// 可选的搜索过滤条件
    pub filter: Option<PersonFilter>,
}

/// 人物搜索过滤条件
///
/// 用于细化人物搜索结果，目前支持按职业筛选
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonFilter {
    /// 可选，按职业筛选（如仅搜索声优或漫画家）
    pub career: Option<Vec<String>>,
}

/// 人物详细信息结构体
///
/// 包含人物的完整信息，在基础信息之上增加了详细简介、个人属性等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonDetail {
    /// 人物唯一标识符
    pub id: u32,
    /// 人物姓名
    pub name: String,
    /// 人物类型（个人、企业或团体）
    pub r#type: PersonType,
    /// 人物职业列表
    pub career: Vec<PersonCareer>,
    /// 人物图片信息
    pub images: SimpleImage,
    /// 详细简介
    pub summary: String,
    /// 是否被锁定（内容不可编辑）
    pub locked: bool,
    /// 最后修改时间（时间戳字符串）
    pub last_modified: String,
    /// 信息框（包含人物详细属性的键值对列表）
    pub infobox: Vec<serde_json::Value>,
    /// 性别（可选，如"男"、"女"）
    pub gender: Option<String>,
    /// 血型（可选）
    pub blood_type: Option<BloodType>,
    /// 出生年份（可选）
    pub birth_year: Option<u32>,
    /// 出生月份（可选）
    pub birth_mon: Option<u32>,
    /// 出生日期（可选）
    pub birth_day: Option<u32>,
    /// 统计信息（使用Box包装以优化内存布局）
    pub stat: Box<Stat>,
}

/// 人物关联的角色信息
///
/// 表示人物参与配音或创作的角色，包含角色基本信息及所属作品信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonCharacter {
    /// 角色ID
    pub id: u32,
    /// 角色名称
    pub name: String,
    /// 角色类型
    pub r#type: CharacterType,
    /// 角色图片信息
    pub images: SimpleImage,
    /// 所属条目ID
    pub subject_id: u32,
    /// 所属条目类型
    pub subject_type: SubjectType,
    /// 所属条目原名
    pub subject_name: String,
    /// 所属条目中文名
    pub subject_name_cn: String,
    /// 人物在该角色中的职位（如"配音"）
    pub staff: String,
}

/// 人物参与的条目信息
///
/// 表示人物参与制作的作品，包含条目基本信息及人物在该作品中的职位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonSubject {
    /// 条目ID
    pub id: u32,
    /// 条目类型
    pub r#type: SubjectType,
    /// 人物在该条目中的职位（如"导演"、"编剧"）
    pub staff: String,
    /// 条目原名
    pub name: String,
    /// 条目中文名
    pub name_cn: String,
    /// 条目图片信息（可选）
    pub images: Option<Image>,
}
