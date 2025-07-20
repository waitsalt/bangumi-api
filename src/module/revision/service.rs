use anyhow::Result;
use reqwest::Method;

use crate::{common::model::BangumiClient, module::model::Paged};

use super::model::{
    RevisionCharacter, RevisionCommon, RevisionEpisode, RevisionPerson, RevisionSubject,
};

impl BangumiClient {
    /// 获取指定人物的修订记录列表
    ///
    /// 查询某个人物的所有编辑修订历史，返回分页的通用修订信息
    ///
    /// # 参数
    /// - `person_id`: 人物ID（必需，指定要查询的人物）
    /// - `limit`: 可选，每页返回的修订记录数量
    /// - `offset`: 可选，结果偏移量（用于分页，从0开始）
    ///
    /// # 返回
    /// 返回包含通用修订信息的分页结果（`Paged<RevisionCommon>`）
    pub async fn get_revision_persons(
        &self,
        person_id: u32,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Paged<RevisionCommon>> {
        // 构建人物修订记录列表接口URL
        let url = format!("{}/v0/revisions/persons", self.base_path);

        // 创建GET请求构建器
        let mut request_builder = self.request_builder(Method::GET, &url);

        // 添加必需的人物ID查询参数
        request_builder = request_builder.query(&[("person_id", &person_id)]);
        // 添加分页参数
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }

        // 发送请求并解析分页的通用修订信息
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取单条人物修订记录的详细信息
    ///
    /// 根据修订记录ID查询某个人物修订的完整数据，包含具体的字段变更内容
    ///
    /// # 参数
    /// - `revision_id`: 修订记录ID（必需，指定要查询的具体修订）
    ///
    /// # 返回
    /// 返回包含人物修订详情的`RevisionPerson`结构体
    pub async fn get_revision_person(&self, revision_id: u32) -> Result<RevisionPerson> {
        // 构建单条人物修订详情接口URL
        let url = format!("{}/v0/revisions/persons/{revision_id}", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析人物修订详情
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取指定角色的修订记录列表
    ///
    /// 查询某个角色的所有编辑修订历史，返回分页的通用修订信息
    ///
    /// # 参数
    /// - `character_id`: 角色ID（必需，指定要查询的角色）
    /// - `limit`: 可选，每页返回的修订记录数量
    /// - `offset`: 可选，结果偏移量（用于分页，从0开始）
    ///
    /// # 返回
    /// 返回包含通用修订信息的分页结果（`Paged<RevisionCommon>`）
    pub async fn get_revision_characters(
        &self,
        character_id: u32,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Paged<RevisionCommon>> {
        // 构建角色修订记录列表接口URL
        let url = format!("{}/v0/revisions/characters", self.base_path);

        // 创建GET请求构建器
        let mut request_builder = self.request_builder(Method::GET, &url);

        // 添加必需的角色ID查询参数
        request_builder = request_builder.query(&[("character_id", &character_id)]);
        // 添加分页参数
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }

        // 发送请求并解析分页的通用修订信息
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取单条角色修订记录的详细信息
    ///
    /// 根据修订记录ID查询某个角色修订的完整数据，包含具体的字段变更内容
    ///
    /// # 参数
    /// - `revision_id`: 修订记录ID（必需，指定要查询的具体修订）
    ///
    /// # 返回
    /// 返回包含角色修订详情的`RevisionCharacter`结构体
    pub async fn get_revision_character(&self, revision_id: u32) -> Result<RevisionCharacter> {
        // 构建单条角色修订详情接口URL
        let url = format!("{}/v0/revisions/characters/{revision_id}", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析角色修订详情
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取指定条目的修订记录列表
    ///
    /// 查询某个条目（作品）的所有编辑修订历史，返回分页的通用修订信息
    ///
    /// # 参数
    /// - `subject_id`: 条目ID（必需，指定要查询的作品）
    /// - `limit`: 可选，每页返回的修订记录数量
    /// - `offset`: 可选，结果偏移量（用于分页，从0开始）
    ///
    /// # 返回
    /// 返回包含通用修订信息的分页结果（`Paged<RevisionCommon>`）
    pub async fn get_revision_subjects(
        &self,
        subject_id: u32,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Paged<RevisionCommon>> {
        // 构建条目修订记录列表接口URL
        let url = format!("{}/v0/revisions/subjects", self.base_path);

        // 创建GET请求构建器
        let mut request_builder = self.request_builder(Method::GET, &url);

        // 添加必需的条目ID查询参数
        request_builder = request_builder.query(&[("subject_id", &subject_id)]);
        // 添加分页参数
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        // 发送请求并解析分页的通用修订信息
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取单条条目修订记录的详细信息
    ///
    /// 根据修订记录ID查询某个条目修订的完整数据，包含具体的字段变更内容
    ///
    /// # 参数
    /// - `revision_id`: 修订记录ID（必需，指定要查询的具体修订）
    ///
    /// # 返回
    /// 返回包含条目修订详情的`RevisionSubject`结构体
    pub async fn get_revision_subject(&self, revision_id: u32) -> Result<RevisionSubject> {
        // 构建单条条目修订详情接口URL
        let url = format!("{}/v0/revisions/subjects/{revision_id}", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析条目修订详情
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取指定剧集的修订记录列表
    ///
    /// 查询某个剧集的所有编辑修订历史，返回分页的通用修订信息
    ///
    /// # 参数
    /// - `episode_id`: 剧集ID（必需，指定要查询的剧集）
    /// - `limit`: 可选，每页返回的修订记录数量
    /// - `offset`: 可选，结果偏移量（用于分页，从0开始）
    ///
    /// # 返回
    /// 返回包含通用修订信息的分页结果（`Paged<RevisionCommon>`）
    pub async fn get_revision_episodes(
        &self,
        episode_id: u32,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Paged<RevisionCommon>> {
        // 构建剧集修订记录列表接口URL
        let url = format!("{}/v0/revisions/episodes", self.base_path);

        // 创建GET请求构建器
        let mut request_builder = self.request_builder(Method::GET, &url);

        // 添加必需的剧集ID查询参数
        request_builder = request_builder.query(&[("episode_id", &episode_id)]);
        // 添加分页参数
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        // 发送请求并解析分页的通用修订信息
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取单条剧集修订记录的详细信息
    ///
    /// 根据修订记录ID查询某个剧集修订的完整数据，包含具体的字段变更内容
    ///
    /// # 参数
    /// - `revision_id`: 修订记录ID（必需，指定要查询的具体修订）
    ///
    /// # 返回
    /// 返回包含剧集修订详情的`RevisionEpisode`结构体
    pub async fn get_revision_episode(&self, revision_id: u32) -> Result<RevisionEpisode> {
        // 构建单条剧集修订详情接口URL
        let url = format!("{}/v0/revisions/episodes/{revision_id}", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析剧集修订详情
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
