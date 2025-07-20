use anyhow::Result;
use bytes::Bytes;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::model::{Paged, SimpleImageType},
};

use super::model::{Character, CharacterPerson, CharacterSearch, CharacterSubject};

impl BangumiClient {
    /// 搜索角色
    ///
    /// 执行角色搜索请求，支持分页和过滤条件
    ///
    /// # 参数
    /// - `limit`: 每页返回的结果数量
    /// - `offset`: 结果偏移量，用于分页
    /// - `payload`: 搜索条件，包含关键词和过滤选项
    ///
    /// # 返回
    /// 返回一个包含角色列表的分页结果
    pub async fn search_characters(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        payload: Option<CharacterSearch>,
    ) -> Result<Paged<Character>> {
        let url = format!("{}/v0/search/characters", self.base_path);

        let mut request_builder = self.request_builder(Method::POST, &url);

        // 添加分页参数
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        // 添加搜索条件
        request_builder = request_builder.json(&payload);

        // 发送请求并解析响应
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取角色详情
    ///
    /// 根据角色ID获取角色的详细信息
    ///
    /// # 参数
    /// - `character_id`: 角色ID
    ///
    /// # 返回
    /// 返回包含角色详细信息的结构体
    pub async fn get_character(&self, character_id: u32) -> Result<Character> {
        let url = format!("{}/v0/characters/{character_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析响应
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取角色图片
    ///
    /// 根据角色ID和图片类型获取角色图片的二进制数据
    ///
    /// # 参数
    /// - `character_id`: 角色ID
    /// - `r#type`: 图片类型
    ///
    /// # 返回
    /// 返回图片的二进制数据
    pub async fn get_character_image(
        &self,
        character_id: u32,
        r#type: SimpleImageType,
    ) -> Result<Bytes> {
        let url = format!("{}/v0/characters/{character_id}/image", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);
        // 添加图片类型参数
        request_builder = request_builder.query(&[("type", &r#type)]);

        // 发送请求并获取图片字节数据
        let res = self.request_send(request_builder).await?.bytes().await?;

        Ok(res)
    }

    /// 获取角色参与的条目
    ///
    /// 根据角色ID获取该角色参与的所有条目信息
    ///
    /// # 参数
    /// - `character_id`: 角色ID
    ///
    /// # 返回
    /// 返回角色参与的条目列表
    pub async fn get_character_subjects(&self, character_id: u32) -> Result<Vec<CharacterSubject>> {
        let url = format!("{}/v0/characters/{character_id}/subjects", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析响应
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取角色的配音演员或创作者
    ///
    /// 根据角色ID获取为该角色配音或创作的人员信息
    ///
    /// # 参数
    /// - `character_id`: 角色ID
    ///
    /// # 返回
    /// 返回角色的配音演员或创作者列表
    pub async fn get_character_persons(&self, character_id: u32) -> Result<Vec<CharacterPerson>> {
        let url = format!("{}/v0/characters/{character_id}/persons", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析响应
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 收藏角色
    ///
    /// 将指定ID的角色添加到当前用户的收藏列表中
    ///
    /// # 参数
    /// - `character_id`: 角色ID
    ///
    /// # 返回
    /// 操作成功返回Ok(())，失败返回错误
    pub async fn collect_character(&self, character_id: u32) -> Result<()> {
        let url = format!("{}/v0/characters/{character_id}/collect", self.base_path);

        let request_builder = self.request_builder(Method::POST, &url);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }

    /// 取消收藏角色
    ///
    /// 从当前用户的收藏列表中移除指定ID的角色
    ///
    /// # 参数
    /// - `character_id`: 角色ID
    ///
    /// # 返回
    /// 操作成功返回Ok(())，失败返回错误
    pub async fn uncollect_character(&self, character_id: u32) -> Result<()> {
        let url = format!("{}/v0/characters/{character_id}/collect", self.base_path);

        let request_builder = self.request_builder(Method::DELETE, &url);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }
}
