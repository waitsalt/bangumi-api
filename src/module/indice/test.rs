use crate::{
    common::model::BangumiClient,
    module::indice::model::{IndexBasicInfo, IndexSubjectAddInfo, IndexSubjectEditInfo},
};

#[tokio::test]
async fn test_add_index() {
    let client = BangumiClient::default();
    let result = client.add_index().await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_index() {
    let client = BangumiClient::default();
    let result = client.get_index(1).await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}

#[tokio::test]
async fn test_edit_index() {
    let client = BangumiClient::default();
    let result = client
        .edit_index(
            1,
            Some(IndexBasicInfo {
                title: Some("title".to_string()),
                description: Some("description".to_string()),
            }),
        )
        .await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_index_subjects() {
    let client = BangumiClient::default();
    let result = client.get_index_subjects(1, None, None, None).await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}

#[tokio::test]
async fn test_add_index_subject() {
    let client = BangumiClient::default();
    let result = client
        .add_index_subject(
            1,
            Some(IndexSubjectAddInfo {
                subject_id: Some(1),
                sort: Some(2),
                comment: Some("comment".to_string()),
            }),
        )
        .await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}

#[tokio::test]
async fn test_edit_index_subject() {
    let client = BangumiClient::default();
    let result = client
        .edit_index_subject(
            1,
            1,
            Some(IndexSubjectEditInfo {
                sort: Some(2),
                comment: Some("comment".to_string()),
            }),
        )
        .await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}

#[tokio::test]
async fn test_delete_index_subject() {
    let client = BangumiClient::default();
    let result = client.delete_index_subject(1, 1).await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}

#[tokio::test]
async fn test_collect_index() {
    let client = BangumiClient::default();
    let result = client.collect_index(1).await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}

#[tokio::test]
async fn test_uncollect_index() {
    let client = BangumiClient::default();
    let result = client.uncollect_index(1).await;
    assert!(result.is_ok(), "失败: {:?}", result.err());
}
