use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::module::{
    character::model::CharacterType,
    collection::model::CollectionStats,
    model::{Image, InfoBox, SimpleImage},
    person::model::{Person, PersonCareer, PersonType},
};

/// 条目类型枚举，用于区分不同类型的内容条目
///
/// 基于u8存储，对应不同类别的内容分类
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SubjectType {
    /// 书籍
    Book = 1,
    /// 动画
    Anime = 2,
    /// 音乐
    Music = 3,
    /// 游戏
    Game = 4,
    /// 三次元
    Real = 6,
}

/// 内容条目的核心数据结构，包含条目的详细信息
///
/// 存储各类内容（动画、书籍等）的完整属性信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
    /// 条目的唯一标识符
    pub id: u32,
    /// 条目的类型（如动画、书籍等）
    pub r#type: SubjectType,
    /// 条目的原始名称（通常为外文原名）
    pub name: String,
    /// 条目的中文名称
    pub name_cn: String,
    /// 条目的详细简介
    pub summary: String,
    /// 是否为系列作品
    pub series: bool,
    /// 是否包含不适宜内容（Not Safe For Work）
    pub nsfw: bool,
    /// 是否被锁定（禁止编辑）
    pub locked: bool,
    /// 条目相关日期（如发布日期、首播日期等，可选）
    pub date: Option<String>,
    /// 条目对应的平台（如游戏平台、播放平台等）
    pub platform: String,
    /// 条目的图片资源信息
    pub images: Image,
    /// 条目的信息框内容（结构化属性）
    pub infobox: InfoBox,
    /// 卷数（主要用于书籍类条目）
    pub volumes: u32,
    /// 集数（主要用于动画、剧集类条目）
    pub eps: u32,
    // pub total_episodes: u32, // 文档中存在 实际不存在
    /// 条目的评分信息
    pub rating: Rating,
    /// 条目的收藏统计数据
    pub collection: CollectionStats,
    /// 元标签列表（系统级标签）
    pub meta_tags: Vec<String>,
    /// 用户标签列表（包含标签统计信息）
    pub tags: Vec<SubjectTag>,
}

/// 条目标签结构体，包含标签信息及统计数据
///
/// 记录用户标记的标签及其相关计数
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubjectTag {
    /// 标签名称
    pub name: String,
    /// 该标签的使用次数
    pub count: u32,
    /// 该标签的总贡献人数
    pub total_cont: u32,
}

/// 每日日历条目结构体，用于展示每日更新的内容
///
/// 按星期分类，包含对应日期的内容条目列表
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DailyCalendarItem {
    /// 星期信息（多语言表示）
    pub weekday: Weekday,
    /// 当日的内容条目列表（精简信息）
    pub items: Vec<SubjectSmall>,
}

/// 星期信息结构体，包含多语言表示及标识
///
/// 提供星期的多语言名称和唯一标识
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Weekday {
    /// 英文星期名称
    pub en: String,
    /// 中文星期名称
    pub cn: String,
    /// 日文星期名称
    pub ja: String,
    /// 星期标识ID（1-7）
    pub id: u8,
}

/// 精简的条目信息结构体，用于列表展示
///
/// 包含条目核心信息，适用于列表、日历等场景
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubjectSmall {
    /// 条目的唯一标识符
    pub id: u32,
    /// 条目的访问链接
    pub url: String,
    /// 条目的类型
    pub r#type: SubjectType,
    /// 条目的原始名称
    pub name: String,
    /// 条目的中文名称
    pub name_cn: String,
    /// 条目的简介
    pub summary: String,
    /// 播出/发布日期
    pub air_date: String,
    /// 播出星期（对应Weekday的id）
    pub air_weekday: u8,
    /// 条目的评分信息（可选）
    pub rating: Option<SubjectSmallRating>,
    /// 条目的排名（可选）
    pub rank: Option<u32>,
    /// 条目的图片资源信息
    pub images: Image,
    // 文档中存在 实际不存在
    // pub eps: u32,
    // pub eps_count: u32,
    /// 条目的收藏统计数据（可选）
    pub collection: Option<CollectionStats>,
}

/// 精简的条目评分信息结构体
///
/// 用于列表场景中的评分展示
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubjectSmallRating {
    /// 总评分人数
    pub total: u32,
    /// 各评分等级的投票数量
    pub count: RatingCount,
    /// 平均分数
    pub score: f64,
}

/// 评分分布统计结构体
///
/// 记录1-10分每个分数段的投票数量
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RatingCount {
    #[serde(rename = "1")]
    pub param_1: u32, // 1分的投票数量
    #[serde(rename = "2")]
    pub param_2: u32, // 2分的投票数量
    #[serde(rename = "3")]
    pub param_3: u32, // 3分的投票数量
    #[serde(rename = "4")]
    pub param_4: u32, // 4分的投票数量
    #[serde(rename = "5")]
    pub param_5: u32, // 5分的投票数量
    #[serde(rename = "6")]
    pub param_6: u32, // 6分的投票数量
    #[serde(rename = "7")]
    pub param_7: u32, // 7分的投票数量
    #[serde(rename = "8")]
    pub param_8: u32, // 8分的投票数量
    #[serde(rename = "9")]
    pub param_9: u32, // 9分的投票数量
    #[serde(rename = "10")]
    pub param_10: u32, // 10分的投票数量
}

/// 条目搜索参数结构体
///
/// 用于构建条目搜索请求，包含关键词、排序和过滤条件
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubjectSearch {
    /// 搜索关键词
    pub keyword: String,
    /// 排序方式（可选）
    pub sort: Option<SubjectSearchSort>,
    /// 过滤条件（可选）
    pub filter: Option<SubjectSearchFilter>,
}

/// 条目搜索排序方式枚举
///
/// 定义搜索结果的排序规则
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubjectSearchSort {
    Match, // 按匹配度排序
    Heat,  // 按热度排序
    Rank,  // 按排名排序
    Score, // 按评分排序
}

/// 条目搜索过滤条件结构体
///
/// 用于精确筛选搜索结果
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubjectSearchFilter {
    /// 条目类型过滤（多选）
    pub r#type: Vec<SubjectType>,
    /// 元标签过滤（多选）
    pub meta_tags: Vec<String>,
    /// 标签过滤（多选）
    pub tag: Vec<String>,
    /// 播出日期过滤（多选）
    pub air_date: Vec<String>,
    /// 评分过滤（多选）
    pub rating: Vec<String>,
    /// 排名过滤（多选）
    pub rank: Vec<String>,
    /// 是否包含不适宜内容
    pub nsfw: bool,
}

/// 完整的条目评分信息结构体
///
/// 包含评分统计的详细数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rating {
    /// 条目排名
    pub rank: u32,
    /// 总评分人数
    pub total: u32,
    /// 各评分等级的投票数量
    pub count: RatingCount,
    /// 平均分数
    pub score: f64,
}

/// 条目子分类枚举（无标签联合类型）
///
/// 根据条目主类型区分不同的子分类体系
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubjectCategory {
    Book(SubjectBookCategory),   // 书籍类条目的子分类
    Anime(SubjectAnimeCategory), // 动画类条目的子分类
    Game(SubjectGameCategory),   // 游戏类条目的子分类
    Real(SubjectRealCategory),   // 真人影视类条目的子分类
}

/// 书籍类条目的子分类枚举
///
/// 基于u16存储，细分书籍类型
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SubjectBookCategory {
    Other = 0,      // 其他书籍类型
    Comic = 1001,   // 漫画
    Novel = 1002,   // 小说
    Artbook = 1003, // 画册/艺术书
}

/// 动画类条目的子分类枚举
///
/// 基于u16存储，细分动画类型
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SubjectAnimeCategory {
    Other = 0, // 其他动画类型
    Tv = 1,    // 电视动画
    Ova = 2,   // OVA（原创动画录像带）
    Movie = 3, // 动画电影
    Web = 5,   // 网络动画
}

/// 游戏类条目的子分类枚举
///
/// 基于u16存储，细分游戏类型
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SubjectGameCategory {
    Other = 0,        // 其他游戏类型
    Game = 4001,      // 普通游戏
    Software = 4002,  // 软件
    Expansion = 4003, // 扩展包
    BoardGame = 4005, // 桌游
}

/// 真人影视类条目的子分类枚举
///
/// 基于u16存储，细分真人影视类型
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SubjectRealCategory {
    Other = 0,             // 其他真人类型
    JapaneseDrama = 1,     // 日剧
    EuroAmericanDrama = 2, // 欧美剧
    ChineseDrama = 3,      // 中剧
    TvDrama = 6001,        // 电视剧
    Movie = 6002,          // 电影
    Performance = 6003,    // 表演/演出
    VarietyShow = 6004,    // 综艺节目
}

/// 条目浏览排序方式枚举
///
/// 定义条目列表的排序规则
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubjectBrowseSort {
    Date, // 按日期排序
    Rank, // 按排名排序
}

/// 与条目相关的人物信息结构体
///
/// 记录参与条目的人物及其关联信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectPerson {
    /// 人物唯一标识符
    pub id: u32,
    /// 人物名称
    pub name: String,
    /// 人物类型
    pub r#type: PersonType,
    /// 人物职业列表
    pub career: Vec<PersonCareer>,
    /// 人物图片资源信息
    pub images: SimpleImage,
    /// 人物与条目的关系（如导演、声优等）
    pub relation: String,
    /// 参与的集数信息
    pub eps: String,
}

/// 与条目相关的角色信息结构体
///
/// 记录条目中的角色及其配音演员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectCharacter {
    /// 角色唯一标识符
    pub id: u32,
    /// 角色名称
    pub name: String,
    /// 角色类型
    pub r#type: CharacterType,
    /// 角色图片资源信息
    pub images: SimpleImage,
    /// 角色与条目的关系
    pub relation: String,
    /// 配音演员列表
    pub actors: Vec<Person>,
}

/// 与条目相关的其他条目信息结构体
///
/// 记录关联条目（如系列作品、衍生作品等）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectSubject {
    /// 关联条目的唯一标识符
    pub id: u32,
    /// 关联条目的类型
    pub r#type: SubjectType,
    /// 关联条目的原始名称
    pub name: String,
    /// 关联条目的中文名称
    pub name_cn: String,
    /// 关联条目的图片资源信息
    pub images: Image,
    /// 与当前条目的关系描述
    pub relation: String,
}
