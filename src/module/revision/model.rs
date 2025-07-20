use serde::{Deserialize, Serialize};

/// 所有修订记录的通用基础结构
///
/// 包含各类修订记录共有的核心字段，用于描述一次修订的基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionCommon {
    /// 修订记录的唯一标识符
    pub id: u32,
    /// 修订类型标识（用于区分不同实体的修订，如人物、角色、条目等）
    pub r#type: u32,
    /// 修订创建者信息（可选，可能为None表示匿名或信息未记录）
    pub creator: Option<Creator>,
    /// 修订摘要（简要描述本次修订的内容或目的）
    pub summary: String,
    /// 修订创建时间（格式通常为ISO 8601时间字符串）
    pub created_at: String,
}

/// 修订创建者信息结构体
///
/// 记录发起修订操作的用户基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    /// 创建者的用户名（登录账号）
    pub username: String,
    /// 创建者的昵称（显示名称）
    pub nickname: String,
}

/// 人物相关修订记录的结构体
///
/// 继承通用修订字段，并包含人物特有的修订数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionPerson {
    /// 修订记录ID
    pub id: u32,
    /// 修订类型标识（人物修订的类型编码）
    pub r#type: u32,
    /// 修订创建者信息（可选）
    pub creator: Option<Creator>,
    /// 修订摘要
    pub summary: String,
    /// 修订创建时间
    pub created_at: String,
    /// 修订的具体数据（可选，键通常为修订版本对比标识，值为对应版本的人物数据）
    pub data: Option<std::collections::HashMap<String, RevisionPersonDataItem>>,
}

/// 人物修订的具体数据项
///
/// 包含人物修订中涉及的详细字段变更信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionPersonDataItem {
    /// 人物信息框内容（结构化的人物属性描述）
    pub prsn_infobox: String,
    /// 人物简介内容
    pub prsn_summary: String,
    /// 人物职业信息（记录职业相关的修订）
    pub profession: PersonRevisionProfession,
    /// 额外信息（如图片等）
    pub extra: RevisionExtra,
    /// 人物名称
    pub prsn_name: String,
}

/// 人物修订中的职业变更信息
///
/// 记录各类职业是否有修订（值可能为修订说明或空字符串）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRevisionProfession {
    /// 制作人职业的修订信息（可选）
    pub producer: Option<String>,
    /// 漫画家职业的修订信息（可选）
    pub mangaka: Option<String>,
    /// 艺术家职业的修订信息（可选）
    pub artist: Option<String>,
    /// 声优职业的修订信息（可选）
    pub seiyu: Option<String>,
    /// 编剧职业的修订信息（可选）
    pub writer: Option<String>,
    /// 插画师职业的修订信息（可选）
    pub illustrator: Option<String>,
    /// 演员职业的修订信息（可选）
    pub actor: Option<String>,
}

/// 修订中的额外信息结构体
///
/// 用于存储各类修订中可能涉及的附加数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionExtra {
    /// 图片相关的修订信息（可选，可能为图片URL或修改说明）
    pub img: Option<String>,
}

/// 角色相关修订记录的结构体
///
/// 继承通用修订字段，并包含角色特有的修订数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionCharacter {
    /// 修订记录ID
    pub id: u32,
    /// 修订类型标识（角色修订的类型编码）
    pub r#type: u32,
    /// 修订创建者信息（可选）
    pub creator: Option<Creator>,
    /// 修订摘要
    pub summary: String,
    /// 修订创建时间
    pub created_at: String,
    /// 修订的具体数据（可选，键通常为修订版本对比标识，值为对应版本的角色数据）
    pub data: Option<std::collections::HashMap<String, RevisionCharacterDataItem>>,
}

/// 角色修订的具体数据项
///
/// 包含角色修订中涉及的详细字段变更信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionCharacterDataItem {
    /// 角色信息框内容
    pub infobox: String,
    /// 角色简介内容
    pub summary: String,
    /// 角色名称
    pub name: String,
    /// 额外信息（如图片等）
    pub extra: RevisionExtra,
}

/// 条目（作品）相关修订记录的结构体
///
/// 继承通用修订字段，并包含条目特有的修订数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionSubject {
    /// 修订记录ID
    pub id: u32,
    /// 修订类型标识（条目修订的类型编码）
    pub r#type: u32,
    /// 修订创建者信息（可选）
    pub creator: Option<Creator>,
    /// 修订摘要
    pub summary: String,
    /// 修订创建时间
    pub created_at: String,
    /// 修订的具体数据（可选，包含条目的详细修订内容）
    pub data: Option<RevisionSubjectData>,
}

/// 条目修订的具体数据
///
/// 包含条目修订中涉及的详细字段变更信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionSubjectData {
    /// 集数相关字段的修订值
    pub field_eps: u32,
    /// 条目信息框内容
    pub field_infobox: String,
    /// 条目简介内容
    pub field_summary: String,
    /// 条目原名
    pub name: String,
    /// 条目中文名
    pub name_cn: String,
    /// 平台标识（如游戏平台、播放平台等）
    pub platform: u32,
    /// 条目ID（修订对应的条目唯一标识）
    pub subject_id: u32,
    /// 条目类型标识
    pub r#type: u32,
    /// 类型ID（更细分的条目类型）
    pub type_id: u32,
    /// 投票相关字段内容
    pub vote_field: String,
}

/// 剧集相关修订记录的结构体
///
/// 继承通用修订字段，并包含剧集特有的修订数据（数据结构更灵活）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionEpisode {
    /// 修订记录ID
    pub id: u32,
    /// 修订类型标识（剧集修订的类型编码）
    pub r#type: u32,
    /// 修订创建者信息（可选）
    pub creator: Option<Creator>,
    /// 修订摘要
    pub summary: String,
    /// 修订创建时间
    pub created_at: String,
    /// 修订的具体数据（可选，使用JSON值存储灵活的结构）
    pub data: Option<serde_json::Value>,
}
