use anyhow::Result;
use bytes::Bytes;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::{model::SimpleImagesType, subject::model::RelatedSubject},
};

use super::model::{PagedPerson, PersonCharacter, PersonDetail, SearchPersonsRequest};

impl BangumiClient {
    pub async fn search_persons(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        search_persons_request: Option<SearchPersonsRequest>,
    ) -> Result<PagedPerson> {
        let url = format!("{}/v0/search/persons", self.base_path);

        let mut request_builder = self.request_builder(Method::POST, &url);

        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        request_builder = request_builder.json(&search_persons_request);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_person_by_id(&self, person_id: i32) -> Result<PersonDetail> {
        let url = format!("{}/v0/persons/{person_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_person_image_by_id(
        &self,
        person_id: i32,
        r#type: SimpleImagesType,
    ) -> Result<Bytes> {
        let url = format!("{}/v0/persons/{person_id}/image", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);
        request_builder = request_builder.query(&[("type", &r#type)]);

        let res = self.request_send(request_builder).await?.bytes().await?;

        Ok(res)
    }

    pub async fn get_related_subjects_by_person_id(
        &self,
        person_id: i32,
    ) -> Result<Vec<RelatedSubject>> {
        let url = format!("{}/v0/persons/{person_id}/subjects", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_related_characters_by_person_id(
        &self,
        person_id: i32,
    ) -> Result<Vec<PersonCharacter>> {
        let url = format!("{}/v0/persons/{person_id}/characters", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn collect_person_by_person_id(&self, person_id: i32) -> Result<()> {
        let url = format!("{}/v0/persons/{person_id}/collect", self.base_path);

        let request_builder = self.request_builder(Method::POST, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn uncollect_person_by_person_id(&self, person_id: i32) -> Result<()> {
        let url = format!("{}/v0/persons/{person_id}/collect", self.base_path);

        let request_builder = self.request_builder(Method::DELETE, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
