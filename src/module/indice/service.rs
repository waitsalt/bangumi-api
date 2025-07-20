use anyhow::Result;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::{
        model::Paged,
        subject::model::{Subject, SubjectType},
    },
};

use super::model::{
    Index, IndexBasicInfo, IndexSubject, IndexSubjectAddInfo, IndexSubjectEditInfo,
};

impl BangumiClient {
    /// 创建新索引
    ///
    /// 用于在Bangumi平台创建一个新的索引（通常是主题性的条目集合）
    ///
    /// # 返回
    /// 返回创建成功的索引详情
    pub async fn add_index(&self) -> Result<Index> {
        // 构建创建索引的API URL
        let url = format!("{}/v0/indices", self.base_path);

        // 创建POST请求构建器
        let request_builder = self.request_builder(Method::POST, &url);

        // 发送请求并解析响应为Index结构体
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取索引详情
    ///
    /// 根据索引ID查询指定索引的完整信息，包括标题、描述、统计数据等
    ///
    /// # 参数
    /// - `index_id`: 索引ID（必需，指定要查询的索引）
    ///
    /// # 返回
    /// 返回包含索引详细信息的Index结构体
    pub async fn get_index(&self, index_id: u32) -> Result<Index> {
        // 构建索引详情API URL
        let url = format!("{}/v0/indices/{index_id}", self.base_path);

        // 创建GET请求构建器
        let request_builder = self.request_builder(Method::GET, &url);

        // 发送请求并解析响应为Index结构体
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 编辑索引基本信息
    ///
    /// 更新指定索引的标题和描述等基本信息
    ///
    /// # 参数
    /// - `index_id`: 索引ID（必需，指定要编辑的索引）
    /// - `payload`: 可选，包含要更新的标题和描述信息
    ///
    /// # 返回
    /// 返回更新后的索引详情
    pub async fn edit_index(
        &self,
        index_id: u32,
        payload: Option<IndexBasicInfo>,
    ) -> Result<Index> {
        // 构建索引编辑API URL
        let url = format!("{}/v0/indices/{index_id}", self.base_path);

        // 创建PUT请求构建器并添加请求体
        let mut request_builder = self.request_builder(Method::PUT, &url);
        request_builder = request_builder.json(&payload);

        // 发送请求并解析响应为更新后的Index结构体
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取索引中的条目列表
    ///
    /// 查询指定索引包含的所有条目，支持按条目类型筛选和分页
    ///
    /// # 参数
    /// - `index_id`: 索引ID（必需，指定要查询的索引）
    /// - `r#type`: 可选，按条目类型筛选（如动画、书籍等）
    /// - `limit`: 可选，每页返回的结果数量
    /// - `offset`: 可选，结果偏移量（用于分页）
    ///
    /// # 返回
    /// 返回包含条目列表的分页结果
    pub async fn get_index_subjects(
        &self,
        index_id: u32,
        r#type: Option<SubjectType>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Paged<IndexSubject>> {
        // 构建索引条目列表API URL
        let url = format!("{}/v0/indices/{index_id}/subjects", self.base_path);

        // 创建GET请求构建器
        let mut request_builder = self.request_builder(Method::GET, &url);

        // 添加条目类型筛选参数
        if let Some(ref param_value) = r#type {
            request_builder = request_builder.query(&[("type", &param_value)]);
        }
        // 添加分页参数
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }

        // 发送请求并解析响应为分页的Subject列表
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 向索引添加条目
    ///
    /// 向指定索引中添加新条目，并可设置排序权重和备注
    ///
    /// # 参数
    /// - `index_id`: 索引ID（必需，指定目标索引）
    /// - `payload`: 可选，包含要添加的条目ID、排序权重和备注
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn add_index_subject(
        &self,
        index_id: u32,
        payload: Option<IndexSubjectAddInfo>,
    ) -> Result<()> {
        // 构建索引条目添加API URL
        let url = format!("{}/v0/indices/{index_id}/subjects", self.base_path);

        // 创建POST请求构建器并添加请求体
        let mut request_builder = self.request_builder(Method::POST, &url);
        request_builder = request_builder.json(&payload);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }

    /// 编辑索引中的条目信息
    ///
    /// 修改指定索引中已有条目的排序权重和备注
    ///
    /// # 参数
    /// - `index_id`: 索引ID（必需，指定目标索引）
    /// - `subject_id`: 条目ID（必需，指定要编辑的条目）
    /// - `payload`: 可选，包含要更新的排序权重和备注
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn edit_index_subject(
        &self,
        index_id: u32,
        subject_id: u32,
        payload: Option<IndexSubjectEditInfo>,
    ) -> Result<()> {
        // 构建索引条目编辑API URL
        let url = format!(
            "{}/v0/indices/{index_id}/subjects/{subject_id}",
            self.base_path
        );

        // 创建PUT请求构建器并添加请求体
        let mut request_builder = self.request_builder(Method::PUT, &url);
        request_builder = request_builder.json(&payload);

        // 发送请求并忽略响应内容
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 从索引中删除条目
    ///
    /// 将指定条目从索引中移除
    ///
    /// # 参数
    /// - `index_id`: 索引ID（必需，指定目标索引）
    /// - `subject_id`: 条目ID（必需，指定要删除的条目）
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn delete_index_subject(&self, index_id: u32, subject_id: u32) -> Result<()> {
        // 构建索引条目删除API URL
        let url = format!(
            "{}/v0/indices/{index_id}/subjects/{subject_id}",
            self.base_path
        );

        // 创建DELETE请求构建器
        let request_builder = self.request_builder(Method::DELETE, &url);

        // 发送请求并忽略响应内容
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 收藏索引
    ///
    /// 将指定索引添加到当前用户的收藏列表
    ///
    /// # 参数
    /// - `index_id`: 索引ID（必需，指定要收藏的索引）
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn collect_index(&self, index_id: u32) -> Result<()> {
        // 构建索引收藏API URL
        let url = format!("{}/v0/indices/{index_id}/collect", self.base_path);

        // 创建POST请求构建器
        let request_builder = self.request_builder(Method::POST, &url);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }

    /// 取消收藏索引
    ///
    /// 将指定索引从当前用户的收藏列表中移除
    ///
    /// # 参数
    /// - `index_id`: 索引ID（必需，指定要取消收藏的索引）
    ///
    /// # 返回
    /// 操作成功返回空结果，失败返回错误
    pub async fn uncollect_index(&self, index_id: u32) -> Result<()> {
        // 构建取消索引收藏API URL
        let url = format!("{}/v0/indices/{index_id}/collect", self.base_path);

        // 创建DELETE请求构建器
        let request_builder = self.request_builder(Method::DELETE, &url);

        // 发送请求并忽略响应内容
        let _res = self.request_send(request_builder).await?;

        Ok(())
    }
}
