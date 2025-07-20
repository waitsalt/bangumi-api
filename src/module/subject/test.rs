use crate::{
    common::model::BangumiClient,
    module::{
        model::ImageType,
        subject::model::{
            SubjectAnimeCategory, SubjectBrowseSort, SubjectCategory, SubjectSearch, SubjectType,
        },
    },
};

/// 测试获取每日放送
#[tokio::test]
async fn test_get_calendar() {
    let client = BangumiClient::default();
    let result = client.get_calendar().await;
    assert!(result.is_ok(), "获取每日放送失败: {:?}", result.err());
}

#[tokio::test]
async fn test_search_subjects() {
    let client = BangumiClient::default();
    // 构造一个简单的搜索请求
    let search_request = Some(SubjectSearch {
        keyword: "eva".to_string(),
        sort: None,
        filter: None,
    });
    let result = client
        .search_subjects(Some(10), Some(0), search_request)
        .await;
    assert!(result.is_ok(), "搜索条目失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_subjects() {
    let client = BangumiClient::default();
    // 测试基本调用（只提供必填参数）
    let result = client
        .get_subjects(
            SubjectType::Anime, // 必填参数
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok(), "获取条目列表失败: {:?}", result.err());

    // 测试带过滤条件的调用
    let result_with_filters = client
        .get_subjects(
            SubjectType::Anime,
            Some(SubjectCategory::Anime(SubjectAnimeCategory::Tv)), // 只查询TV动画
            Some(true),                                             // 只查询系列动画
            Some("TV"),                                             // 平台
            Some(SubjectBrowseSort::Rank),                          // 按排名排序
            Some(2023),                                             // 2023年的动画
            Some(4),                                                // 4月
            Some(10),                                               // 每页10条
            Some(0),                                                // 第一页
        )
        .await;
    assert!(
        result_with_filters.is_ok(),
        "带过滤条件的查询失败: {:?}",
        result_with_filters.err()
    );
}

#[tokio::test]
async fn test_get_subject() {
    let client = BangumiClient::default();
    let result = client.get_subject(1024).await;
    assert!(result.is_ok(), "获取条目失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_subject_image() {
    let client = BangumiClient::default();
    let result = client.get_subject_image(1027, ImageType::Large).await;
    assert!(result.is_ok(), "获取条目图片失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_subject_persons() {
    let client = BangumiClient::default();
    let result = client.get_subject_persons(1024).await;
    assert!(result.is_ok(), "获取相关人物失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_subject_characters() {
    let client = BangumiClient::default();
    let result = client.get_subject_characters(1024).await;
    assert!(result.is_ok(), "获取角色列表失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_subject_subjects() {
    let client = BangumiClient::default();
    let result = client.get_subject_subjects(1024).await;
    assert!(result.is_ok(), "获取关联条目失败: {:?}", result.err());
}
