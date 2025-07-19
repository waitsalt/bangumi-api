use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Debug)]
pub struct Paged<T> {
    /// 总条目数
    pub total: u32,
    /// 每页条目数
    pub limit: u32,
    /// 偏移量
    pub offset: u32,
    /// 当前页的数据列表
    pub data: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Images {
    pub large: String,
    pub common: String,
    pub medium: String,
    pub small: String,
    pub grid: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImagesType {
    Large,
    Common,
    Medium,
    Small,
    Grid,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SimpleImages {
    pub large: String,
    pub medium: String,
    pub small: String,
    pub grid: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SimpleImagesType {
    Large,
    Medium,
    Small,
    Grid,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchSort {
    Match,
    Heat,
    Rank,
    Score,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub comments: i32,
    pub collects: i32,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BloodType {
    A = 1,
    B = 2,
    AB = 3,
    O = 4,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub count: u32,
}
