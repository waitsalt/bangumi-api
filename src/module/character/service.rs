use anyhow::Result;
use bytes::Bytes;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::{model::SimpleImagesType, subject::model::SubjectRelation},
};

use super::model::{Character, CharacterPerson, PagedCharacter, SearchCharactersRequest};

impl BangumiClient {
    pub async fn search_characters(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        search_characters_request: Option<SearchCharactersRequest>,
    ) -> Result<PagedCharacter> {
        let url = format!("{}/v0/search/characters", self.base_path);

        let mut request_builder = self.request_builder(Method::POST, &url);

        if let Some(ref param_value) = limit {
            request_builder = request_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            request_builder = request_builder.query(&[("offset", &param_value)]);
        }
        request_builder = request_builder.json(&search_characters_request);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_character_by_id(&self, character_id: i32) -> Result<Character> {
        let url = format!("{}/v0/characters/{character_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_character_image_by_id(
        &self,
        character_id: i32,
        r#type: SimpleImagesType,
    ) -> Result<Bytes> {
        let url = format!("{}/v0/characters/{character_id}/image", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);
        request_builder = request_builder.query(&[("type", &r#type)]);

        let res = self.request_send(request_builder).await?.bytes().await?;

        Ok(res)
    }

    pub async fn get_related_subjects_by_character_id(
        &self,
        character_id: i32,
    ) -> Result<Vec<SubjectRelation>> {
        let url = format!("{}/v0/characters/{character_id}/subjects", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_related_persons_by_character_id(
        &self,
        character_id: i32,
    ) -> Result<Vec<CharacterPerson>> {
        let url = format!("{}/v0/characters/{character_id}/persons", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn collect_character_by_character_id(&self, character_id: i32) -> Result<()> {
        let url = format!("{}/v0/characters/{character_id}/collect", self.base_path);

        let request_builder = self.request_builder(Method::POST, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn uncollect_character_by_character(&self, character_id: i32) -> Result<()> {
        let url = format!("{}/v0/characters/{character_id}/collect", self.base_path);

        let request_builder = self.request_builder(Method::DELETE, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
