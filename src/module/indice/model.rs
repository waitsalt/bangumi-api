use serde::{Deserialize, Serialize};

use crate::module::{
    model::{Image, InfoBox, Stat},
    revision::model::Creator,
    subject::model::SubjectType,
};

/// 表示索引的详细信息结构体
///
/// 包含索引的基本属性、统计数据、创建者信息及状态标识，用于完整描述一个索引的全貌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    /// 索引唯一标识符
    pub id: u32,
    /// 索引标题
    pub title: String,
    /// 索引描述（用于说明索引的主题、目的或内容简介）
    pub desc: String,
    /// 索引包含的条目总数（可选，可能为None表示未统计）
    pub total: Option<u32>,
    /// 索引的统计信息（如浏览量、收藏量等）
    pub stat: Stat,
    /// 创建时间
    pub created_at: String,
    /// 最后更新时间（格式同上，记录索引内容最后修改的时间）
    pub updated_at: String,
    /// 索引创建者信息
    pub creator: Creator,
    /// 是否被封禁（true表示索引因违规被隐藏或禁止访问）
    pub ban: bool,
    /// 是否包含成人内容（true表示索引含不适宜未成年人的内容）
    pub nsfw: bool,
}

/// 用于更新索引基本信息的请求参数结构体
///
/// 包含可修改的索引基本属性，字段均为可选，仅提供需要更新的字段即可
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexBasicInfo {
    /// 可选，索引的新标题（若提供则更新标题）
    pub title: Option<String>,
    /// 可选，索引的新描述（若提供则更新描述）
    pub description: Option<String>,
}

/// 用于向索引添加条目的请求参数结构体
///
/// 描述添加到索引中的条目信息，包括条目ID、排序权重和备注
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSubjectAddInfo {
    /// 可选，要添加到索引的条目ID（必需参数，未提供则添加失败）
    pub subject_id: Option<u32>,
    /// 可选，该条目在索引中的排序权重（数值越小越靠前，默认可能按添加顺序）
    pub sort: Option<u32>,
    /// 可选，对该条目添加的备注说明（用于解释条目在索引中的意义）
    pub comment: Option<String>,
}

/// 用于编辑索引中已有条目的请求参数结构体
///
/// 用于修改索引中已有条目的排序权重和备注信息，不支持修改条目ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSubjectEditInfo {
    /// 可选，更新该条目在索引中的排序权重
    pub sort: Option<u32>,
    /// 可选，更新对该条目的备注说明
    pub comment: Option<String>,
}

/// 索引中条目的基本信息
///
/// 包含索引基本属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSubject {
    /// 条目的唯一标识符
    pub id: u32,
    /// 条目的类型（如动画、书籍等）
    pub r#type: SubjectType,
    /// 条目的原始名称（通常为外文原名）
    pub name: String,
    /// 条目的中文名称
    pub name_cn: String,
    /// 条目相关日期（如发布日期、首播日期等，可选）
    pub date: Option<String>,
    /// 条目的图片资源信息
    pub images: Image,
    /// 条目的信息框内容（结构化属性）
    pub infobox: InfoBox,
    // 条目添加到索引的时间
    pub add_at: Option<String>,
    // 条目在索引中的备注
    pub comment: String,
}
