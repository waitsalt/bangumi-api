use anyhow::Result;
use bytes::Bytes;
use reqwest::Method;

use crate::common::model::BangumiClient;

use super::model::{AvatarType, User, UserPublic};

impl BangumiClient {
    pub async fn get_user(&self, username: &str) -> Result<UserPublic> {
        let url = format!("{}/v0/users/{username}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    pub async fn get_user_avatar(&self, username: &str, r#type: AvatarType) -> Result<Bytes> {
        let url = format!("{}/v0/users/{username}/avatar", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);
        request_builder = request_builder.query(&[("type", r#type)]);

        let res = self.request_send(request_builder).await?.bytes().await?;

        Ok(res)
    }

    pub async fn get_me(&self) -> Result<User> {
        let url = format!("{}/v0/me", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
