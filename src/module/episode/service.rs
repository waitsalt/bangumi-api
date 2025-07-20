use anyhow::Result;
use reqwest::Method;

use crate::{common::model::BangumiClient, module::model::Paged};

use super::model::{Episode, EpisodeType};

impl BangumiClient {
    /// 获取指定条目的剧集列表
    ///
    /// 支持按剧集类型筛选，并提供分页功能，用于获取某部作品的所有剧集信息
    ///
    /// # 参数
    /// - `subject_id`: 条目ID（必需，指定要查询的作品）
    /// - `r#type`: 可选，按剧集类型筛选（如普通剧集、SP、OP等）
    /// - `limit`: 可选，每页返回的结果数量
    /// - `offset`: 可选，结果偏移量（用于分页，从0开始）
    ///
    /// # 返回
    /// 返回包含剧集列表的分页结果，每页数据为`Episode`结构体数组
    pub async fn get_episodes(
        &self,
        subject_id: u32,
        r#type: Option<EpisodeType>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Paged<Episode>> {
        // 构建剧集列表接口URL
        let url = format!("{}/v0/episodes", self.base_path);

        // 创建GET请求构建器
        let mut request_builder = self.request_builder(Method::GET, &url);

        // 添加必需的条目ID查询参数（指定查询哪个作品的剧集）
        request_builder = request_builder.query(&[("subject_id", &subject_id)]);
        // 可选：添加剧集类型筛选参数
        if let Some(ref param_value) = r#type {
            request_builder = request_builder.query(&[("type", &param_value)]);
        }
        // 可选：添加分页参数（每页数量）
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        // 可选：添加分页参数（偏移量）
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }

        // 发送请求并解析响应为分页的剧集列表
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取单个剧集的详细信息
    ///
    /// 根据剧集ID查询某一具体剧集的完整信息，包括名称、时长、播出日期等
    ///
    /// # 参数
    /// - `episode_id`: 剧集ID（必需，指定要查询的具体剧集）
    ///
    /// # 返回
    /// 返回包含该剧集详细信息的`Episode`结构体
    pub async fn get_episode(&self, episode_id: u32) -> Result<Episode> {
        // 构建单个剧集详情接口URL
        let url = format!("{}/v0/episodes/{episode_id}", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析响应为剧集详情结构体
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
