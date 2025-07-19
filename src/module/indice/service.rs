use anyhow::Result;
use reqwest::Method;

use crate::{common::model::BangumiClient, module::subject::model::SubjectType};

use super::model::{Index, IndexBasicInfo, IndexSubjectAddInfo, IndexSubjectEditInfo};

impl BangumiClient {
    pub async fn new_index(&self) -> Result<Index> {
        let url = format!("{}/v0/indices", self.base_path);

        let request_builder = self.request_builder(Method::POST, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_index_by_id(&self, index_id: i32) -> Result<Index> {
        let url = format!("{}/v0/indices/{index_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn edit_index_by_id(
        &self,
        index_id: i32,
        index_basic_info: Option<IndexBasicInfo>,
    ) -> Result<Index> {
        let url = format!("{}/v0/indices/{index_id}", self.base_path);

        let mut request_builder = self.request_builder(Method::PUT, &url);
        request_builder = request_builder.json(&index_basic_info);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_index_subjects_by_index_id(
        &self,
        index_id: i32,
        r#type: Option<SubjectType>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<()> {
        let url = format!("{}/v0/indices/{index_id}/subjects", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        if let Some(ref param_value) = r#type {
            request_builder = request_builder.query(&[("type", &param_value)]);
        }
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn add_subject_to_index_by_index_id(
        &self,
        index_id: i32,
        index_subject_add_info: Option<IndexSubjectAddInfo>,
    ) -> Result<()> {
        let url = format!("{}/v0/indices/{index_id}/subjects", self.base_path);

        let mut request_builder = self.request_builder(Method::POST, &url);

        request_builder = request_builder.json(&index_subject_add_info);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn edit_index_subjects_by_index_id_and_subject_id(
        &self,
        index_id: i32,
        subject_id: i32,
        index_subject_edit_info: Option<IndexSubjectEditInfo>,
    ) -> Result<()> {
        let url = format!(
            "{}/v0/indices/{index_id}/subjects/{subject_id}",
            self.base_path
        );

        let mut request_builder = self.request_builder(Method::PUT, &url);

        request_builder = request_builder.json(&index_subject_edit_info);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn delelte_subject_from_index_by_index_id_and_subject_id(
        &self,
        index_id: i32,
        subject_id: i32,
    ) -> Result<()> {
        let url = format!(
            "{}/v0/indices/{index_id}/subjects/{subject_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::DELETE, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn collect_index_by_index_id_and_user_id(&self, index_id: i32) -> Result<()> {
        let url = format!("{}/v0/indices/{index_id}/collect", self.base_path);

        let request_builder = self.request_builder(Method::POST, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn uncollect_index_by_index_id_and_user_id(&self, index_id: i32) -> Result<()> {
        let url = format!("{}/v0/indices/{index_id}/collect", self.base_path);

        let request_builder = self.request_builder(Method::DELETE, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
