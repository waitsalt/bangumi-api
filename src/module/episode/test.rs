use crate::common::model::BangumiClient;

#[tokio::test]
async fn test_get_episodes() {
    let client = BangumiClient::default();
    let result = client.get_episodes(2, None, None, None).await;
    assert!(result.is_ok(), "获取分集列表失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_episode_by_id() {
    let client = BangumiClient::default();
    let result = client.get_episode(2).await;
    assert!(result.is_ok(), "获取分集详情失败: {:?}", result.err());
}
