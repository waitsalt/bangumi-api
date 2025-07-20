use serde::{Deserialize, Serialize};

/// Bangumi 统一错误响应结构体
///
/// 当 Bangumi 的 API 调用失败时，后端会返回该格式的错误信息。
/// 该结构体可直接用于反序列化 JSON 错误响应，方便在客户端进行错误处理。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BangumiError {
    /// 错误的标题，用于快速定位问题类型
    /// 例如："Unauthorized"、"ValidationError" 等
    pub title: String,

    /// 详细的错误信息，包含具体的错误原因、请求路径和方法
    pub details: BangumiErrordetails,

    /// 可选的请求 ID
    /// 用于在日志系统中追踪某一次具体的请求，便于调试和排查问题
    pub request_id: Option<String>,

    /// 错误的详细描述，通常是一段可读性强的文字
    /// 用于向用户展示具体的错误原因或建议的解决方式
    pub description: String,
}

/// Bangumi 错误详情结构体
///
/// 包含本次请求的具体信息，帮助开发者快速定位问题发生的上下文。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BangumiErrordetails {
    /// 具体的错误信息字符串
    /// 例如："user not found"、"invalid token" 等
    /// 如果为 None，则表示未提供具体的错误信息
    pub error: Option<String>,

    /// 请求的路径
    /// 例如："/api/user/info"
    pub path: String,

    /// 请求的 HTTP 方法
    /// 例如："GET"、"POST" 等
    pub method: String,
}
