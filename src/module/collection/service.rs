use anyhow::Result;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::{
        episode::model::EpisodeType, model::Paged, subject::model::SubjectType,
        user::model::UserSubjectCollection,
    },
};

use super::model::{
    CollectionType, PagedUserCharacterCollection, PagedUserCollection, PagedUserPersonCollection,
    UserCharacterCollection, UserEpisodeCollection, UserSubjectCollectionModifyPayload,
};

impl BangumiClient {
    pub async fn get_user_collections_by_username(
        &self,
        username: &str,
        subject_type: Option<SubjectType>,
        r#type: Option<CollectionType>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PagedUserCollection> {
        let url = format!("{}/v0/users/{username}/collections", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        if let Some(ref param_value) = subject_type {
            request_builder = request_builder.query(&[("subject_type", &param_value)]);
        }
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

    pub async fn get_user_collection(
        &self,
        username: &str,
        subject_id: i32,
    ) -> Result<UserSubjectCollection> {
        let url = format!(
            "{}/v0/users/{username}/collections/{subject_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn post_user_collection(
        &self,
        subject_id: i32,
        user_subject_collection_modify_payload: Option<UserSubjectCollectionModifyPayload>,
    ) -> Result<PagedUserCollection> {
        let url = format!("{}/v0/users/-/collections/{subject_id}", self.base_path);

        let mut request_builder = self.request_builder(Method::POST, &url);
        request_builder = request_builder.json(&user_subject_collection_modify_payload);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn patch_user_collection(
        &self,
        subject_id: i32,
        user_subject_collection_modify_payload: Option<UserSubjectCollectionModifyPayload>,
    ) -> Result<PagedUserCollection> {
        let url = format!("{}/v0/users/-/collections/{subject_id}", self.base_path);

        let mut request_builder = self.request_builder(Method::PATCH, &url);
        request_builder = request_builder.json(&user_subject_collection_modify_payload);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_user_subject_episode_collection(
        &self,
        subject_id: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        episode_type: Option<EpisodeType>,
    ) -> Result<Paged<UserEpisodeCollection>> {
        let url = format!(
            "{}/v0/users/-/collections/{subject_id}/episodes",
            self.base_path
        );

        let mut request_builder = self.request_builder(Method::POST, &url);

        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = episode_type {
            request_builder = request_builder.query(&[("episode_type", &param_value)]);
        }

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn patch_user_subject_episode_collection(
        &self,
        subject_id: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        episode_type: Option<EpisodeType>,
    ) -> Result<Paged<UserEpisodeCollection>> {
        let url = format!(
            "{}/v0/users/-/collections/{subject_id}/episodes",
            self.base_path
        );

        let mut request_builder = self.request_builder(Method::PATCH, &url);

        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = episode_type {
            request_builder = request_builder.query(&[("episode_type", &param_value)]);
        }

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_user_episode_collection(
        &self,
        episode_id: i32,
    ) -> Result<UserEpisodeCollection> {
        let url = format!(
            "{}/v0/users/-/collections/-/episodes/{episode_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn put_user_episode_collection(
        &self,
        episode_id: i32,
    ) -> Result<UserEpisodeCollection> {
        let url = format!(
            "{}/v0/users/-/collections/-/episodes/{episode_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::PUT, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_user_character_collections(
        &self,
        username: &str,
    ) -> Result<PagedUserCharacterCollection> {
        let url = format!(
            "{}/v0/users/{username}/collections/-/characters",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_user_character_collection(
        &self,
        username: &str,
        character_id: i32,
    ) -> Result<UserCharacterCollection> {
        let url = format!(
            "{}/v0/users/{username}/collections/-/characters/{character_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_user_person_collections(
        &self,
        username: &str,
    ) -> Result<PagedUserPersonCollection> {
        let url = format!(
            "{}/v0/users/{username}/collections/-/persons",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_user_person_collection(
        &self,
        username: &str,
        person_id: i32,
    ) -> Result<PagedUserPersonCollection> {
        let url = format!(
            "{}/v0/users/{username}/collections/-/persons/{person_id}",
            self.base_path
        );

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
