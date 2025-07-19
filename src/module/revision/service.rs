use anyhow::Result;
use reqwest::Method;

use crate::common::model::BangumiClient;

use super::model::{
    CharacterRevision, DetailedRevision, PagedRevision, PersonRevision, SubjectRevision,
};

impl BangumiClient {
    pub async fn get_person_revisions(
        &self,
        person_id: i32,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PagedRevision> {
        let url = format!("{}/v0/revisions/persons", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        request_builder = request_builder.query(&[("person_id", &person_id)]);
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_person_revision_by_revision_id(
        &self,
        revision_id: i32,
    ) -> Result<PersonRevision> {
        let url = format!("{}/v0/revisions/persons/{revision_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_character_revisions(
        &self,
        character_id: i32,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PagedRevision> {
        let url = format!("{}/v0/revisions/characters", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        request_builder = request_builder.query(&[("character_id", &character_id)]);
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_character_revision_by_revision_id(
        &self,
        revision_id: i32,
    ) -> Result<CharacterRevision> {
        let url = format!("{}/v0/revisions/characters/{revision_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_subject_revisions(
        &self,
        subject_id: i32,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PagedRevision> {
        let url = format!("{}/v0/revisions/subjects", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        request_builder = request_builder.query(&[("subject_id", &subject_id)]);
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_subject_revision_by_revision_id(
        &self,
        revision_id: i32,
    ) -> Result<SubjectRevision> {
        let url = format!("{}/v0/revisions/subjects/{revision_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_episode_revisions(
        &self,
        episode_id: i32,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PagedRevision> {
        let url = format!("{}/v0/revisions/episodes", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        request_builder = request_builder.query(&[("episode_id", &episode_id)]);
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_episode_revision_by_revision_id(
        &self,
        revision_id: i32,
    ) -> Result<DetailedRevision> {
        let url = format!("{}/v0/revisions/episodes/{revision_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
