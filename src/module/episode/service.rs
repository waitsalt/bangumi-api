use anyhow::Result;
use reqwest::Method;

use crate::common::model::BangumiClient;

use super::model::{EpisodeDetail, EpisodeType, PagedEpisode};

impl BangumiClient {
    pub async fn get_episodes(
        &self,
        subject_id: u32,
        r#type: Option<EpisodeType>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<PagedEpisode> {
        let url = format!("{}/v0/episodes", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        request_builder = request_builder.query(&[("subject_id", &subject_id)]);
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

    pub async fn get_episode_by_id(&self, episode_id: i32) -> Result<EpisodeDetail> {
        let url = format!("{}/v0/episodes/{episode_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
