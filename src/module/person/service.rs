use anyhow::Result;
use bytes::Bytes;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::model::{Paged, SimpleImageType},
};

use super::model::{PersonCharacter, PersonDetail, PersonSearch, PersonSubject};

impl BangumiClient {
    /// 搜索人物
    ///
    /// 支持按关键词搜索人物，并可通过筛选条件和分页参数控制结果范围
    ///
    /// # 参数
    /// - `limit`: 可选，每页返回的结果数量
    /// - `offset`: 可选，结果偏移量（用于分页，从0开始）
    /// - `payload`: 可选，搜索条件，包含关键词和职业筛选等信息
    ///
    /// # 返回
    /// 返回包含人物详细信息的分页结果
    pub async fn search_persons(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        payload: Option<PersonSearch>,
    ) -> Result<Paged<PersonDetail>> {
        // 构建人物搜索接口URL
        let url = format!("{}/v0/search/persons", self.base_path);

        // 创建POST请求构建器
        let mut request_builder = self.request_builder(Method::POST, &url);

        // 添加分页参数
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        // 添加搜索条件请求体
        request_builder = request_builder.json(&payload);

        // 发送请求并解析分页的人物详情结果
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取人物详细信息
    ///
    /// 根据人物ID查询完整的人物信息，包括基本资料、职业、统计数据等
    ///
    /// # 参数
    /// - `person_id`: 人物ID（必需，指定要查询的人物）
    ///
    /// # 返回
    /// 返回包含人物完整信息的`PersonDetail`结构体
    pub async fn get_person(&self, person_id: u32) -> Result<PersonDetail> {
        // 构建人物详情接口URL
        let url = format!("{}/v0/persons/{person_id}", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析人物详情
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取人物图片
    ///
    /// 根据人物ID和图片类型获取人物图片的二进制数据
    ///
    /// # 参数
    /// - `person_id`: 人物ID（必需，指定目标人物）
    /// - `r#type`: 图片类型（如原图、缩略图等，通过`SimpleImageType`指定）
    ///
    /// # 返回
    /// 返回图片的二进制数据（`Bytes`类型）
    pub async fn get_person_image(&self, person_id: u32, r#type: SimpleImageType) -> Result<Bytes> {
        // 构建人物图片接口URL
        let url = format!("{}/v0/persons/{person_id}/image", self.base_path);

        // 创建GET请求构建器并添加图片类型参数
        let mut request_builder = self.request_builder(Method::GET, &url);
        request_builder = request_builder.query(&[("type", &r#type)]);

        // 发送请求并获取图片二进制数据
        let res = self.request_send(request_builder).await?.bytes().await?;

        Ok(res)
    }

    /// 获取人物参与的条目列表
    ///
    /// 查询指定人物参与制作的所有作品（条目），包含人物在作品中的职位信息
    ///
    /// # 参数
    /// - `person_id`: 人物ID（必需，指定目标人物）
    ///
    /// # 返回
    /// 返回包含人物参与条目的列表（`PersonSubject`结构体数组）
    pub async fn get_person_subjects(&self, person_id: u32) -> Result<Vec<PersonSubject>> {
        // 构建人物参与条目接口URL
        let url = format!("{}/v0/persons/{person_id}/subjects", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析条目列表
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取人物关联的角色列表
    ///
    /// 查询指定人物参与配音或创作的角色，包含角色所属作品信息
    ///
    /// # 参数
    /// - `person_id`: 人物ID（必需，指定目标人物）
    ///
    /// # 返回
    /// 返回包含人物关联角色的列表（`PersonCharacter`结构体数组）
    pub async fn get_person_characters(&self, person_id: u32) -> Result<Vec<PersonCharacter>> {
        // 构建人物关联角色接口URL
        let url = format!("{}/v0/persons/{person_id}/characters", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析角色列表
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 收藏人物
    ///
    /// 将指定人物添加到当前用户的收藏列表（需认证，依赖`access_token`）
    ///
    /// # 参数
    /// - `person_id`: 人物ID（必需，指定要收藏的人物）
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn collect_person(&self, person_id: u32) -> Result<()> {
        // 构建人物收藏接口URL
        let url = format!("{}/v0/persons/{person_id}/collect", self.base_path);

        // 创建POST请求构建器
        let request_builder = self.request_builder(Method::POST, &url);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }

    /// 取消收藏人物
    ///
    /// 将指定人物从当前用户的收藏列表中移除（需认证，依赖`access_token`）
    ///
    /// # 参数
    /// - `person_id`: 人物ID（必需，指定要取消收藏的人物）
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn uncollect_person(&self, person_id: u32) -> Result<()> {
        // 构建取消人物收藏接口URL
        let url = format!("{}/v0/persons/{person_id}/collect", self.base_path);

        // 创建DELETE请求构建器
        let request_builder = self.request_builder(Method::DELETE, &url);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }
}
