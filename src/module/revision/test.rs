use crate::common::model::BangumiClient;

#[tokio::test]
async fn test_get_revision_persons() {
    let client = BangumiClient::default();
    let result = client.get_revision_persons(1, Some(10), Some(0)).await;
    assert!(result.is_ok(), "获取人物修订历史失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_revision_person() {
    let client = BangumiClient::default();
    let result = client.get_revision_person(2081539).await;
    assert!(result.is_ok(), "获取特定人物修订失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_revision_characters() {
    let client = BangumiClient::default();
    let result = client.get_revision_characters(1, Some(10), Some(0)).await;
    assert!(result.is_ok(), "获取角色修订历史失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_revision_character() {
    let client = BangumiClient::default();
    // You may need to replace with a known revision ID
    let result = client.get_revision_character(1).await;
    assert!(result.is_ok(), "获取特定角色修订失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_revision_subjects() {
    let client = BangumiClient::default();
    let result = client.get_revision_subjects(1, Some(10), Some(0)).await;
    assert!(result.is_ok(), "获取条目修订历史失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_revision_subject() {
    let client = BangumiClient::default();
    let result = client.get_revision_subject(1).await;
    assert!(result.is_ok(), "获取特定条目修订失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_revision_episodes() {
    let client = BangumiClient::default();
    let result = client.get_revision_episodes(1, Some(10), Some(0)).await;
    assert!(result.is_ok(), "获取章节修订历史失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_revision_episode() {
    let client = BangumiClient::default();
    // You may need to replace with a known revision ID
    let result = client.get_revision_episode(147359).await;
    assert!(result.is_ok(), "获取特定章节修订失败: {:?}", result.err());
}
