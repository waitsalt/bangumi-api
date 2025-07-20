use anyhow::Result;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::{episode::model::EpisodeType, model::Paged, subject::model::SubjectType},
};

use super::model::{
    CollectionCharacter, CollectionEpisode, CollectionEpisodeUpdate, CollectionEpisodesUpdate,
    CollectionPerson, CollectionSubject, CollectionSubjectUpdate, CollectionType,
};

impl BangumiClient {
    /// 获取用户的收藏条目列表
    ///
    /// 支持按条目类型、收藏状态筛选，并提供分页功能
    ///
    /// # 参数
    /// - `username`: 用户名（目标用户的名称）
    /// - `subject_type`: 可选，按条目类型筛选（如动画、书籍等）
    /// - `r#type`: 可选，按收藏状态筛选（如想看、在看等）
    /// - `limit`: 可选，每页返回结果数量
    /// - `offset`: 可选，结果偏移量（用于分页）
    ///
    /// # 返回
    /// 返回包含收藏条目的分页结果
    pub async fn get_collection_subjects(
        &self,
        username: &str,
        subject_type: Option<SubjectType>,
        r#type: Option<CollectionType>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Paged<CollectionSubject>> {
        // 构建请求URL：用户收藏条目列表接口
        let url = format!("{}/v0/users/{username}/collections", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        // 添加条目类型筛选参数
        if let Some(ref param_value) = subject_type {
            request_builder = request_builder.query(&[("subject_type", &param_value)]);
        }
        // 添加收藏状态筛选参数
        if let Some(ref param_value) = r#type {
            request_builder = request_builder.query(&[("type", &param_value)]);
        }
        // 添加分页参数：每页数量
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        // 添加分页参数：偏移量
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }

        // 发送请求并解析分页结果
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取用户对特定条目的收藏详情
    ///
    /// 查询指定用户对某个条目的收藏状态、评分、评论等信息
    ///
    /// # 参数
    /// - `username`: 用户名
    /// - `subject_id`: 条目ID
    ///
    /// # 返回
    /// 返回该条目对应的收藏详情
    pub async fn get_collection_subject(
        &self,
        username: &str,
        subject_id: u32,
    ) -> Result<CollectionSubject> {
        // 构建请求URL：用户特定条目收藏详情接口
        let url = format!(
            "{}/v0/users/{username}/collections/{subject_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析收藏详情
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 创建或更新当前用户对条目的收藏
    ///
    /// 用于提交或修改用户对指定条目的收藏状态（需认证）
    ///
    /// # 参数
    /// - `subject_id`: 条目ID
    /// - `payload`: 收藏更新参数（包含状态、评分、进度等）
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn post_collection_subject(
        &self,
        subject_id: u32,
        payload: Option<CollectionSubjectUpdate>,
    ) -> Result<()> {
        // 构建请求URL：当前用户条目收藏操作接口（"-"表示当前认证用户）
        let url = format!("{}/v0/users/-/collections/{subject_id}", self.base_path);

        let mut request_builder = self.request_builder(Method::POST, &url);
        // 添加请求体参数
        request_builder = request_builder.json(&payload);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }

    /// 部分更新当前用户对条目的收藏
    ///
    /// 用于增量修改收藏信息（如仅更新评分或进度，需认证）
    ///
    /// # 参数
    /// - `subject_id`: 条目ID
    /// - `payload`: 部分更新参数（可选字段，仅修改指定内容）
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn patch_collection_subject(
        &self,
        subject_id: u32,
        payload: Option<CollectionSubjectUpdate>,
    ) -> Result<()> {
        // 构建请求URL：当前用户条目收藏部分更新接口
        let url = format!("{}/v0/users/-/collections/{subject_id}", self.base_path);

        let mut request_builder = self.request_builder(Method::PATCH, &url);
        // 添加请求体参数
        request_builder = request_builder.json(&payload);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }

    /// 获取当前用户条目的剧集收藏状态
    ///
    /// 查询指定条目下各剧集的收藏/观看状态（需认证）
    ///
    /// # 参数
    /// - `subject_id`: 条目ID
    /// - `offset`: 可选，结果偏移量（分页用）
    /// - `limit`: 可选，每页数量
    /// - `episode_type`: 可选，按剧集类型筛选（如正片、SP等）
    ///
    /// # 返回
    /// 返回包含剧集收藏状态的分页结果
    pub async fn get_collection_episodes(
        &self,
        subject_id: u32,
        offset: Option<u32>,
        limit: Option<u32>,
        episode_type: Option<EpisodeType>,
    ) -> Result<Paged<CollectionEpisode>> {
        // 构建请求URL：当前用户条目剧集收藏接口
        let url = format!(
            "{}/v0/users/-/collections/{subject_id}/episodes",
            self.base_path
        );

        let mut request_builder = self.request_builder(Method::GET, &url);

        // 添加分页参数
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        // 添加剧集类型筛选参数
        if let Some(ref param_value) = episode_type {
            request_builder = request_builder.query(&[("episode_type", &param_value)]);
        }

        // 发送请求并解析分页结果
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 批量更新当前用户剧集收藏状态
    ///
    /// 用于一次性修改多个剧集的收藏状态（需认证）
    ///
    /// # 参数
    /// - `subject_id`: 条目ID
    /// - `payload`: 批量更新参数（包含剧集ID列表和目标状态）
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn patch_collection_episodes(
        &self,
        subject_id: u32,
        payload: Option<CollectionEpisodesUpdate>,
    ) -> Result<()> {
        // 构建请求URL：当前用户剧集批量收藏更新接口
        let url = format!(
            "{}/v0/users/-/collections/{subject_id}/episodes",
            self.base_path
        );

        let mut request_builder = self.request_builder(Method::PATCH, &url);
        // 添加请求体参数
        request_builder = request_builder.json(&payload);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }

    /// 获取当前用户单个剧集的收藏状态
    ///
    /// 查询指定剧集的收藏/观看状态（需认证）
    ///
    /// # 参数
    /// - `episode_id`: 剧集ID
    ///
    /// # 返回
    /// 返回该剧集的收藏状态详情
    pub async fn get_collection_episode(&self, episode_id: u32) -> Result<CollectionEpisode> {
        // 构建请求URL：当前用户单个剧集收藏状态接口
        let url = format!(
            "{}/v0/users/-/collections/-/episodes/{episode_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析结果
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 更新当前用户单个剧集的收藏状态
    ///
    /// 修改指定剧集的收藏/观看状态（需认证）
    ///
    /// # 参数
    /// - `episode_id`: 剧集ID
    /// - `payload`: 剧集状态更新参数
    ///
    /// # 返回
    /// 返回 Ok(())
    pub async fn put_collection_episode(
        &self,
        episode_id: u32,
        payload: Option<CollectionEpisodeUpdate>,
    ) -> Result<()> {
        // 构建请求URL：当前用户单个剧集收藏更新接口
        let url = format!(
            "{}/v0/users/-/collections/-/episodes/{episode_id}",
            self.base_path
        );

        let mut request_builder = self.request_builder(Method::PUT, &url);
        // 添加请求体参数
        request_builder = request_builder.json(&payload);

        // 发送请求并解析更新后的结果
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }

    /// 获取用户收藏的角色列表
    ///
    /// 查询指定用户收藏的所有角色（支持分页）
    ///
    /// # 参数
    /// - `username`: 用户名
    ///
    /// # 返回
    /// 返回包含收藏角色的分页结果
    pub async fn get_collection_characters(
        &self,
        username: &str,
    ) -> Result<Paged<CollectionCharacter>> {
        // 构建请求URL：用户角色收藏列表接口
        let url = format!(
            "{}/v0/users/{username}/collections/-/characters",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析分页结果
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取用户对特定角色的收藏详情
    ///
    /// 查询指定用户对某个角色的收藏信息
    ///
    /// # 参数
    /// - `username`: 用户名
    /// - `character_id`: 角色ID
    ///
    /// # 返回
    /// 返回该角色的收藏详情
    pub async fn get_collection_character(
        &self,
        username: &str,
        character_id: u32,
    ) -> Result<CollectionCharacter> {
        // 构建请求URL：用户特定角色收藏详情接口
        let url = format!(
            "{}/v0/users/{username}/collections/-/characters/{character_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析结果
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取用户收藏的人物列表
    ///
    /// 查询指定用户收藏的所有人物（如配音演员、导演等，支持分页）
    ///
    /// # 参数
    /// - `username`: 用户名
    ///
    /// # 返回
    /// 返回包含收藏人物的分页结果
    pub async fn get_collection_persons(&self, username: &str) -> Result<Paged<CollectionPerson>> {
        // 构建请求URL：用户人物收藏列表接口
        let url = format!(
            "{}/v0/users/{username}/collections/-/persons",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析分页结果
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取用户对特定人物的收藏详情
    ///
    /// 查询指定用户对某个人物的收藏信息
    ///
    /// # 参数
    /// - `username`: 用户名
    /// - `person_id`: 人物ID
    ///
    /// # 返回
    /// 返回该人物的收藏详情
    pub async fn get_collection_person(
        &self,
        username: &str,
        person_id: u32,
    ) -> Result<CollectionPerson> {
        // 构建请求URL：用户特定人物收藏详情接口
        let url = format!(
            "{}/v0/users/{username}/collections/-/persons/{person_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析结果
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
