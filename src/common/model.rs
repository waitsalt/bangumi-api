use anyhow::{Result, bail};
use reqwest::StatusCode;

use super::error::BangumiError;

pub struct BangumiClient {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub access_token: Option<String>,
}

impl Default for BangumiClient {
    fn default() -> Self {
        BangumiClient {
            base_path: "https://api.bgm.tv".to_string(),
            user_agent: Some(
                "waitsalt/bangumi-api (https://github.com/waitsalt/bangumi-api)".to_string(),
            ),
            client: reqwest::Client::new(),
            access_token: None,
        }
    }
}

impl BangumiClient {
    pub fn request_builder(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        let mut builder = self.client.request(method, url);

        if let Some(ua) = &self.user_agent {
            builder = builder.header(reqwest::header::USER_AGENT, ua);
        }

        if let Some(token) = &self.access_token {
            builder = builder.bearer_auth(token);
        }

        builder
    }

    pub async fn request_send(
        &self,
        request_builder: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        let response = request_builder.send().await?;
        let status_code = response.status();
        match status_code {
            StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => {
                let res: BangumiError = response.json().await?;
                bail!(format!("请求错误: {:?}", res))
            }
            _ => {
                let res = response.error_for_status()?;
                return Ok(res);
            }
        }
    }
}
