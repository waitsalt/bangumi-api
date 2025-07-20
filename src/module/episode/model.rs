use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 表示剧集的详细信息结构体
///
/// 包含剧集的基本属性、播出信息、关联条目等完整数据，用于描述单个剧集的具体内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// 剧集唯一标识符
    pub id: u32,
    /// 剧集类型（如普通剧集、SP、OP等）
    pub r#type: EpisodeType,
    /// 剧集原名（通常为日文或外文名称）
    pub name: String,
    /// 剧集中文名
    pub name_cn: String,
    /// 排序权重，用于确定剧集在列表中的展示顺序
    pub sort: f64,
    /// 集数编号（可能为小数，如特别篇可能标记为1.5）
    pub ep: f64,
    /// 播出日期（格式通常为YYYY-MM-DD，可能为空字符串表示未公开）
    pub airdate: String,
    /// 该剧集的评论数量
    pub comment: u32,
    /// 时长描述（如"24分钟"，用于直观展示）
    pub duration: String,
    /// 剧集简介/描述
    pub desc: String,
    /// 所属光盘编号（用于多光盘发行的剧集）
    pub disc: u32,
    /// 关联的条目ID（该剧集所属的作品ID）
    pub subject_id: u32,
    /// 时长（秒数），用于精确的时长计算
    pub duration_seconds: u32,
}

/// 剧集类型枚举
///
/// 用于区分不同类型的剧集内容，底层使用u8存储以节省空间
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EpisodeType {
    /// 普通剧集（正片内容）
    Normal = 0,
    /// 特别篇（Special Episode，额外内容）
    SP = 1,
    /// 片头曲（Opening Theme，片头动画）
    OP = 2,
    /// 片尾曲（Ending Theme，片尾动画）
    ED = 3,
}
