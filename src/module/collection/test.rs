use crate::{
    common::model::BangumiClient,
    module::{
        collection::model::{
            CollectionEpisodeType, CollectionEpisodeUpdate, CollectionEpisodesUpdate,
            CollectionSubjectUpdate, CollectionType,
        },
        episode::model::EpisodeType,
        subject::model::SubjectType,
    },
};

#[tokio::test]
async fn test_get_collection_subjects() {
    let client = BangumiClient::default();
    let result = client
        .get_collection_subjects(
            "sai",
            Some(SubjectType::Anime),
            Some(CollectionType::Doing),
            Some(10),
            Some(0),
        )
        .await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_get_collection_subject() {
    let client = BangumiClient::default();
    let result = client.get_collection_subject("1056427", 23161).await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_post_collection_subject() {
    let client = BangumiClient::default();
    let payload = CollectionSubjectUpdate {
        r#type: Some(CollectionType::Done),
        rate: None,
        ep_status: None,
        vol_status: None,
        comment: None,
        private: None,
        tags: None,
    };
    let result = client.post_collection_subject(123, Some(payload)).await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_patch_collection_subject() {
    let client = BangumiClient::default();
    let payload = CollectionSubjectUpdate {
        r#type: Some(CollectionType::Wish),
        rate: None,
        ep_status: None,
        vol_status: None,
        comment: None,
        private: None,
        tags: None,
    };
    let result = client.patch_collection_subject(123, Some(payload)).await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_get_collection_episodes() {
    let client = BangumiClient::default();
    let result = client
        .get_collection_episodes(123, Some(0), Some(10), Some(EpisodeType::Normal))
        .await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_patch_collection_episodes() {
    let client = BangumiClient::default();
    let result = client
        .patch_collection_episodes(
            123,
            Some(CollectionEpisodesUpdate {
                episode_id: vec![1, 2, 3],
                r#type: CollectionEpisodeType::Done,
            }),
        )
        .await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_get_collection_episode() {
    let client = BangumiClient::default();
    let result = client.get_collection_episode(456).await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_put_collection_episode() {
    let client = BangumiClient::default();
    let result = client
        .put_collection_episode(
            123,
            Some(CollectionEpisodeUpdate {
                r#type: CollectionEpisodeType::Done,
            }),
        )
        .await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_get_collection_characters() {
    let client = BangumiClient::default();
    let result = client.get_collection_characters("sai").await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_get_collection_character() {
    let client = BangumiClient::default();
    let result = client.get_collection_character("sai", 1).await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_get_collection_persons() {
    let client = BangumiClient::default();
    let result = client.get_collection_persons("sai").await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}

#[tokio::test]
async fn test_get_collection_person() {
    let client = BangumiClient::default();
    let result = client.get_collection_person("sai", 1).await;
    assert!(result.is_ok(), "错误: {:?}", result.err());
}
