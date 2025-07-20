use anyhow::{Result, bail};

use super::error::BangumiError;

/// 用于与Bangumi API进行交互的客户端
///
/// 提供了构建请求、发送请求以及处理响应的功能
pub struct BangumiClient {
    /// API的基础路径
    pub base_path: String,
    /// 可选的User-Agent头，用于标识请求来源
    pub user_agent: Option<String>,
    /// reqwest HTTP客户端，用于发送实际的网络请求
    pub client: reqwest::Client,
    /// 可选的访问令牌，用于认证需要授权的API请求
    pub access_token: Option<String>,
}

/// 为BangumiClient提供默认实现
///
/// 默认配置：
/// - 使用"https://api.bgm.tv"作为基础路径
/// - 设置特定格式的User-Agent
/// - 初始化一个基本的reqwest客户端
/// - 不包含访问令牌
impl Default for BangumiClient {
    fn default() -> Self {
        BangumiClient {
            base_path: "https://api.bgm.tv".to_string(),
            user_agent: Some(
                "waitsalt/bangumi-api (https://github.com/waitsalt/bangumi-api)".to_string(),
            ),
            client: reqwest::Client::new(),
            access_token: Some("lshFF34IO3sCl6Ra6EXpoqeWTr5ARhI8DUSUgcCy".to_string()),
        }
    }
}

impl BangumiClient {
    /// 创建一个新的BangumiClient实例
    ///
    /// # 参数
    /// - `base_path`: API的基础路径
    /// - `user_agent`: 可选的User-Agent头
    /// - `access_token`: 可选的访问令牌
    ///
    /// # 返回
    /// 返回一个使用指定参数配置的BangumiClient实例
    pub fn new(
        base_path: String,
        user_agent: Option<String>,
        access_token: Option<String>,
    ) -> Self {
        BangumiClient {
            base_path,
            user_agent,
            client: reqwest::Client::new(),
            access_token,
        }
    }

    /// 构建HTTP请求
    ///
    /// 根据提供的HTTP方法和URL创建一个RequestBuilder实例，并设置必要的请求头
    ///
    /// # 参数
    /// - `method`: HTTP请求方法
    /// - `url`: 请求的URL
    ///
    /// # 返回
    /// 返回一个配置了User-Agent和Bearer Token(如果有)的RequestBuilder
    pub fn request_builder(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        let mut builder = self.client.request(method, url);

        // 设置User-Agent头(如果有)
        if let Some(ua) = &self.user_agent {
            builder = builder.header(reqwest::header::USER_AGENT, ua);
        }

        // 设置Bearer Token(如果有)
        if let Some(token) = &self.access_token {
            builder = builder.bearer_auth(token);
        }

        builder
    }

    /// 发送HTTP请求并处理响应
    ///
    /// 执行RequestBuilder构建的请求，并根据响应状态码进行相应处理
    ///
    /// # 参数
    /// - `request_builder`: 包含请求信息的RequestBuilder
    ///
    /// # 返回
    /// - 如果请求成功(状态码200-299)，返回包含响应的Result
    /// - 如果请求失败，将响应体解析为BangumiError并返回错误
    pub async fn request_send(
        &self,
        request_builder: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        // 发送请求并获取响应
        let response = request_builder.send().await?;
        // 获取响应状态码
        let status_code = response.status();
        // 判断请求是否成功
        match status_code.is_success() {
            true => {
                // 请求成功，返回响应
                return Ok(response);
            }
            false => {
                // 请求失败，将响应体解析为错误信息
                let res: BangumiError = response.json().await?;
                bail!(format!("请求错误: {:?}", res))
            }
        }
    }
}
