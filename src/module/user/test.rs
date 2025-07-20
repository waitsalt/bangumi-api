use crate::{common::model::BangumiClient, module::user::model::AvatarType};

#[tokio::test]
async fn test_get_user() {
    let client = BangumiClient::default();
    let result = client.get_user("sai").await;
    assert!(result.is_ok(), "获取用户信息失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_user_avatar() {
    let client = BangumiClient::default();
    let result = client.get_user_avatar("sai", AvatarType::Large).await;
    assert!(result.is_ok(), "获取用户头像失败: {:?}", result.err());
}

// 需要认证的测试
#[tokio::test]
async fn test_get_me() {
    let client = BangumiClient::default();
    let result = client.get_me().await;
    assert!(result.is_ok(), "获取当前用户信息失败: {:?}", result.err());
}
