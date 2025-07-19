use anyhow::Result;
use bytes::Bytes;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::{character::model::RelatedCharacter, model::ImagesType, person::model::RelatedPerson},
};

use super::model::{
    DailyCalendarItem, PagedSubject, SearchSubjectsRequest, Subject, SubjectCategory,
    SubjectRelation, SubjectSort, SubjectType,
};

impl BangumiClient {
    pub async fn get_calendar(&self) -> Result<Vec<DailyCalendarItem>> {
        let url = format!("{}/calendar", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn search_subjects(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        search_subjects_request: Option<SearchSubjectsRequest>,
    ) -> Result<PagedSubject> {
        let url = format!("{}/v0/search/subjects", self.base_path);
        let mut request_builder = self.request_builder(Method::POST, &url);

        if let Some(ref limit) = limit {
            request_builder = request_builder.query(&[("limit", limit)]);
        }

        if let Some(ref offset) = offset {
            request_builder = request_builder.query(&[("offset", offset)]);
        }

        let request_builder = request_builder.json(&search_subjects_request);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_subjects(
        &self,
        r#type: SubjectType,
        cat: Option<SubjectCategory>,
        series: Option<bool>,
        platform: Option<&str>,
        sort: Option<SubjectSort>,
        year: Option<u32>,
        month: Option<u32>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<PagedSubject> {
        let url = format!("{}/v0/subjects", self.base_path);
        let mut req_builder = self.request_builder(reqwest::Method::GET, &url);

        req_builder = req_builder.query(&[("type", &r#type)]);
        if let Some(ref param_value) = cat {
            req_builder = req_builder.query(&[("cat", &param_value)]);
        }
        if let Some(ref param_value) = series {
            req_builder = req_builder.query(&[("series", &param_value)]);
        }
        if let Some(ref param_value) = platform {
            req_builder = req_builder.query(&[("platform", &param_value)]);
        }
        if let Some(ref param_value) = sort {
            req_builder = req_builder.query(&[("sort", &param_value)]);
        }
        if let Some(ref param_value) = year {
            req_builder = req_builder.query(&[("year", &param_value)]);
        }
        if let Some(ref param_value) = month {
            req_builder = req_builder.query(&[("month", &param_value)]);
        }
        if let Some(ref param_value) = limit {
            req_builder = req_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            req_builder = req_builder.query(&[("offset", &param_value)]);
        }

        let res = self.request_send(req_builder).await?.json().await?;
        Ok(res)
    }

    pub async fn get_subject_by_id(&self, subject_id: u32) -> Result<Subject> {
        let url = format!("{}/v0/subjects/{subject_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_subject_image_by_id(
        &self,
        subject_id: u32,
        r#type: ImagesType,
    ) -> Result<Bytes> {
        let url = format!("{}/v0/subjects/{subject_id}/image", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        request_builder = request_builder.query(&[("type", &r#type)]);

        let res = self.request_send(request_builder).await?.bytes().await?;

        Ok(res)
    }

    pub async fn get_related_persons_by_subject_id(
        &self,
        subject_id: u32,
    ) -> Result<Vec<RelatedPerson>> {
        let url = format!("{}/v0/subjects/{subject_id}/persons", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_related_characters_by_subject_id(
        &self,
        subject_id: u32,
    ) -> Result<Vec<RelatedCharacter>> {
        let url = format!("{}/v0/subjects/{subject_id}/characters", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_related_subjects_by_subject_id(
        &self,
        subject_id: u32,
    ) -> Result<Vec<SubjectRelation>> {
        let url = format!("{}/v0/subjects/{subject_id}/subjects", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
